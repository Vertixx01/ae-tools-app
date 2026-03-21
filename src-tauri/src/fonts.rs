use crate::models::{FontAuditResult, FontEntry};
use crate::util::powershell;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub fn audit_fonts(project_path: &str) -> Result<FontAuditResult, String> {
    let path = Path::new(project_path);
    if !path.exists() {
        return Err("Project file not found".into());
    }

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let fonts = if extension.to_lowercase() == "aepx" {
        extract_from_aepx(project_path)?
    } else {
        extract_from_aep(project_path)?
    };

    let installed = get_installed_font_names();
    let mut font_entries = Vec::new();
    let mut missing_count = 0;

    for font_name in fonts {
        // Simple matching logic: exact or case-insensitive
        // Registry names often have "(TrueType)" or similar suffixes
        let is_installed = installed.contains(&font_name) || 
                          installed.iter().any(|ifont| {
                              let lower_ifont = ifont.to_lowercase();
                              let lower_target = font_name.to_lowercase();
                              lower_ifont.contains(&lower_target) || lower_target.contains(&lower_ifont)
                          });
        
        if !is_installed {
            missing_count += 1;
        }

        font_entries.push(FontEntry {
            family: font_name.clone(),
            name: font_name.clone(),
            style: None, 
            is_installed,
        });
    }

    Ok(FontAuditResult {
        success: true,
        project_path: project_path.to_string(),
        fonts: font_entries,
        missing_count,
    })
}

fn extract_from_aepx(path: &str) -> Result<BTreeSet<String>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut fonts = BTreeSet::new();
    
    // After Effects AEPX looks like: <font family="Inter" fontStyle="Bold"/>
    // We'll use a simple loop with line analysis
    for line in content.lines() {
        if line.contains("<font ") && line.contains("family=\"") {
            if let Some(start) = line.find("family=\"") {
                let after = &line[start + 8..];
                if let Some(end) = after.find('"') {
                    let family = after[..end].to_string();
                    if !family.is_empty() {
                        fonts.insert(family);
                    }
                }
            }
        }
    }
    
    Ok(fonts)
}

// ── RIFX / COS Font Parser for AEP files ────────────────────────────────────
// AEP files use RIFX (big-endian RIFF) with "Egg!" form type.
// Font names are stored in COS-encoded (PDF-like) data blocks with the marker
// "/CoolTypeFont". The font name follows as a UTF-16 BE string in parentheses.
// Pattern: /CoolTypeFont /0 << /0 (??FONTNAME) where ?? is a 2-byte BOM/prefix.

/// Needle to search for in the binary data.
const COOL_TYPE_FONT: &[u8] = b"CoolTypeFont";

/// Extract font names from a binary AEP file by searching for /CoolTypeFont markers.
fn extract_from_aep(path: &str) -> Result<BTreeSet<String>, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let mut fonts = BTreeSet::new();

    // Find all occurrences of "CoolTypeFont" in the binary
    let mut search_start = 0;
    while search_start < data.len() {
        // Find the next CoolTypeFont marker
        let pos = match find_bytes(&data, COOL_TYPE_FONT, search_start) {
            Some(p) => p,
            None => break,
        };

        // After the marker, look for the opening parenthesis '(' which starts the font name
        // Typically: CoolTypeFont /0 << /0 (??FONTNAME)
        let scan_end = (pos + 200).min(data.len());
        if let Some(paren_offset) = find_byte(&data, b'(', pos, scan_end) {
            // Skip the 2-byte prefix (BOM/unknown) after the '('
            let name_start = paren_offset + 3; // skip '(' + 2 prefix bytes
            
            if name_start < scan_end {
                // Read UTF-16 BE characters until closing ')'
                let mut utf16_chars: Vec<u16> = Vec::new();
                let mut i = name_start;
                while i + 1 < data.len() {
                    let hi = data[i];
                    let lo = data[i + 1];
                    
                    // Check for closing parenthesis ')' as a UTF-16 char or raw byte
                    if hi == 0 && lo == b')' { break; }
                    if hi == b')' { break; }
                    
                    utf16_chars.push(u16::from_be_bytes([hi, lo]));
                    i += 2;
                }

                if let Ok(name) = String::from_utf16(&utf16_chars) {
                    let clean = name.trim();
                    if !clean.is_empty() && clean != "AdobeInvisFont" {
                        fonts.insert(clean.to_string());
                    }
                }
            }
        }

        search_start = pos + COOL_TYPE_FONT.len();
    }

    // If COS search yielded nothing, fall back to a simple byte-scan for PostScript names
    if fonts.is_empty() {
        let mut i = 0;
        let blocklist = ["Control", "Paint", "FreePin", "Fine-tune", "Pom", "Xom", "Zom", "ADBE"];
        let styles = ["Regular", "Bold", "Italic", "Roman", "Light", "Medium", "Black", "Condensed", "Heavy", "Narrow", "Thin", "Semibold", "Oblique", "MT", "PSMT"];

        while i < data.len().saturating_sub(6) {
            if data[i].is_ascii_uppercase() {
                let start = i;
                let mut end = i;
                while end < data.len() && (data[end].is_ascii_alphanumeric() || data[end] == b'-') {
                    end += 1;
                }
                let len = end - start;
                
                if len >= 6 && len <= 64 && data[start..end].contains(&b'-') {
                    let s = String::from_utf8_lossy(&data[start..end]).to_string();
                    let parts: Vec<&str> = s.split('-').collect();
                    
                    if parts.len() >= 2 {
                        let family = parts[0];
                        let style = parts.last().unwrap_or(&"");
                        
                        // Reject known AE noise patterns
                        let is_noise = blocklist.iter().any(|&b| family.contains(b)) || 
                                     style.contains("sLIST") || 
                                     (style.len() == 4 && style.chars().all(|c| c.is_ascii_digit())); // matches -0000
                        
                        // A confident font name usually has a known style suffix or is a known PostScript format
                        let has_known_style = styles.iter().any(|&st| style.contains(st) || family.contains(st));
                        
                        if !is_noise && (has_known_style || (family.len() >= 3 && style.len() >= 2)) {
                            if s != "AdobeInvisFont" {
                                fonts.insert(s);
                            }
                        }
                    }
                }
                i = end;
            } else {
                i += 1;
            }
        }
    }

    Ok(fonts)
}

/// Find a byte sequence needle in a haystack starting from a given offset.
fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start + needle.len() > haystack.len() { return None; }
    haystack[start..].windows(needle.len()).position(|w| w == needle).map(|p| p + start)
}

/// Find a single byte in a range.
fn find_byte(data: &[u8], byte: u8, start: usize, end: usize) -> Option<usize> {
    let end = end.min(data.len());
    data[start..end].iter().position(|&b| b == byte).map(|p| p + start)
}


fn get_installed_font_names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    
    // Query Registry for installed fonts
    let script = "Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts' | Get-Member -MemberType Properties | Select-Object -ExpandProperty Name";
    if let Ok(output) = powershell(script) {
        for line in output.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && trimmed != "PSPath" && trimmed != "PSParentPath" && trimmed != "PSChildName" && trimmed != "PSDrive" && trimmed != "PSProvider" {
                // Remove suffixes like "(TrueType)" if they exist
                let clean = if let Some(idx) = trimmed.find(" (") {
                    &trimmed[..idx]
                } else {
                    trimmed
                };
                names.insert(clean.to_string());
            }
        }
    }
    
    names
}
