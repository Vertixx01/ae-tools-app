use std::fs;
use std::path::Path;
use std::collections::BTreeSet;

/// Metadata extracted from an AEP project.
#[derive(Default)]
pub struct AepMetadata {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub fps: Option<f32>,
    pub plugins: Vec<String>,
    pub contributions: Vec<String>,
    pub compositions: Vec<String>,
    pub missing_footage: usize,
}

/// RIFX FourCC constants
const RIFX: [u8; 4] = *b"RIFX";
const EGG:  [u8; 4] = *b"Egg!";
const CDTA: [u8; 4] = *b"cdta";
const LIST: [u8; 4] = *b"LIST";
const UTF8: [u8; 4] = *b"Utf8";

pub fn analyze_aep(path: &str) -> AepMetadata {
    let mut meta = AepMetadata::default();
    let Ok(data) = fs::read(path) else { return meta; };

    if data.len() < 12 { return meta; }

    // 1. Basic RIFX check
    if &data[0..4] != RIFX || &data[8..12] != EGG {
        return meta;
    }

    let file_size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let limit = (12 + file_size - 4).min(data.len());

    // 2. Structured Metadata Extraction (cdta/idta)
    walk_chunks(&data, 12, limit, &mut meta);

    // 3. Plugin Audit (Byte-scan for Match Names and 3rd party keywords)
    meta.plugins = audit_plugins(&data);

    meta
}

fn read_be_u32(data: &[u8], pos: usize) -> u32 {
    if pos + 4 > data.len() { return 0; }
    u32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]])
}

fn read_be_u16(data: &[u8], pos: usize) -> u16 {
    if pos + 2 > data.len() { return 0; }
    u16::from_be_bytes([data[pos], data[pos+1]])
}

fn walk_chunks(data: &[u8], start: usize, end: usize, meta: &mut AepMetadata) {
    let mut pos = start;
    let mut found_paths = BTreeSet::new();
    let mut found_comps = BTreeSet::new();

    while pos + 8 <= end {
        let tag = &data[pos..pos+4];
        let size = read_be_u32(data, pos + 4) as usize;
        let chunk_start = pos + 8;
        let chunk_end = (chunk_start + size).min(end);

        if tag == LIST {
            if chunk_start + 4 <= chunk_end {
                walk_chunks(data, chunk_start + 4, chunk_end, meta);
            }
        } else if tag == CDTA {
            // Composition Data
            if size >= 156 {
                let w = read_be_u16(data, chunk_start + 140);
                let h = read_be_u16(data, chunk_start + 142);
                let f = read_be_u16(data, chunk_start + 154); 
                
                if w > 0 && h > 0 {
                    meta.width = Some(w as u32);
                    meta.height = Some(h as u32);
                }
                if f > 0 && f < 1000 {
                   meta.fps = Some(f as f32);
                }
            }
        } else if tag == UTF8 || tag == b"atyp" {
            // Potential path or comp strings
            if chunk_start < chunk_end {
                let s = String::from_utf8_lossy(&data[chunk_start..chunk_end]).to_string();
                let trimmed = s.trim().trim_matches('\0');
                if is_likely_path(trimmed) {
                    found_paths.insert(trimmed.to_string());
                } else if trimmed.len() > 1 && trimmed.len() < 100 && !trimmed.contains('\\') && !trimmed.contains('/') {
                    // This is likely a composition or footage name
                    found_comps.insert(trimmed.to_string());
                }
            }
        }

        let padded = if size % 2 != 0 { size + 1 } else { size };
        pos = chunk_start + padded;
    }

    // Filter and update meta
    meta.compositions.extend(found_comps.into_iter().filter(|c| {
        // Simple heuristic: comps usually aren't numeric or standard metadata tags
        c.chars().any(|ch| ch.is_alphabetic()) && c.len() > 2
    }));

    for path in found_paths {
        if !Path::new(&path).exists() {
            meta.missing_footage += 1;
        }
    }
}

fn is_likely_path(s: &str) -> bool {
    if s.len() < 5 || s.len() > 300 { return false; }
    // Drive letter like C:\ or unc path \\
    let is_drive = s.len() > 2 
        && s.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false) 
        && s.chars().nth(1) == Some(':') 
        && s.chars().nth(2) == Some('\\');

    if is_drive || s.starts_with("\\\\") {
        return true;
    }
    // Check for common footage extensions
    let lower = s.to_lowercase();
    for ext in [".mp4", ".mov", ".avi", ".png", ".jpg", ".jpeg", ".wav", ".mp3", ".psd", ".ai", ".obj", ".c4d"] {
        if lower.ends_with(ext) {
            return true;
        }
    }
    false
}

/// Known 3rd party plugin keywords and match names
const PLUGIN_MATCH_NAMES: &[(&str, &str)] = &[
    ("Saber", "VideoCopilot_Saber"),
    ("Particular", "Trapcode_Particular"),
    ("Deep Glow", "DeepGlow"),
    ("Element 3D", "VideoCopilot_Element"),
    ("Optical Flares", "VideoCopilot_OpticalFlares"),
    ("Plexus", "Plexus"),
    ("Stardust", "Stardust"),
    ("Mocha", "MochaAE"),
    ("Red Giant", "Red Giant"),
];

fn audit_plugins(data: &[u8]) -> Vec<String> {
    let mut found = BTreeSet::new();
    
    // Convert a chunk of the binary to string for scanning
    // This isn't super efficient for HUGE files, but AEPs are usually < 50MB
    // We'll scan specifically for match names.
    let search_data = String::from_utf8_lossy(data);

    for (label, match_name) in PLUGIN_MATCH_NAMES {
        if search_data.contains(match_name) {
            found.insert(label.to_string());
        }
    }

    found.into_iter().collect()
}
