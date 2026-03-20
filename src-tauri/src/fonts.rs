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

fn extract_from_aep(path: &str) -> Result<BTreeSet<String>, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let mut fonts = BTreeSet::new();
    
    // Heuristic scan for common 8-bit text chunks in AEP (ASCII only for now)
    // Most font names are stored as plain ASCII near 'FONT' markers or property blocks
    // This is experimental and may miss many fonts in binary files
    
    let mut i = 0;
    while i < data.len().saturating_sub(4) {
        // Some AEP versions store font names after a specific byte sequence
        // or simply as null-terminated strings.
        // We'll look for strings over 4 chars that look like font identifiers
        if data[i] >= 32 && data[i] <= 126 {
            let mut end = i;
            while end < data.len() && data[end] >= 32 && data[end] <= 126 {
                end += 1;
            }
            if end - i > 4 {
                let s = String::from_utf8_lossy(&data[i..end]);
                // Heuristic: Font names in AEP often have specific capitalization or suffixes
                if s.contains("-Bold") || s.contains("-Regular") || s.contains("-Italic") || s.contains("Condensed") {
                   fonts.insert(s.to_string());
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
    
    Ok(fonts)
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
