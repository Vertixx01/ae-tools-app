use crate::models::{
    ExpressionError, ExpressionAuditResult
};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_expression_logs() -> Vec<ExpressionError> {
    let mut errors = Vec::new();
    let documents = match dirs::document_dir() {
        Some(d) => d,
        None => return errors,
    };

    let adobe_path = documents.join("Adobe").join("After Effects");
    if !adobe_path.exists() {
        return errors;
    }

    if let Ok(entries) = fs::read_dir(adobe_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let log_file = path.join("After Effects Expressions Log.txt");
                if log_file.exists() {
                    if let Ok(content) = fs::read_to_string(&log_file) {
                        errors.extend(parse_expression_log(&content, &path));
                    }
                }
            }
        }
    }

    errors.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    errors
}

fn parse_expression_log(content: &str, version_path: &Path) -> Vec<ExpressionError> {
    let mut errors = Vec::new();
    let version = version_path.file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("Unknown")
        .to_string();

    // The log usually looks like:
    // [Timestamp]
    // Project: <path>
    // Composition: <comp>
    // Layer: <layer>
    // Property: <prop>
    // Error: <msg>
    // --------------------------------------------------

    let blocks = content.split("--------------------------------------------------");
    for block in blocks {
        if block.trim().is_empty() { continue; }

        let mut error = ExpressionError {
            timestamp: String::new(),
            project: None,
            composition: String::new(),
            layer: String::new(),
            property: String::new(),
            message: String::new(),
            version: version.clone(),
        };

        for line in block.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                error.timestamp = line[1..line.len()-1].to_string();
            } else if line.starts_with("Project:") {
                error.project = Some(line["Project:".len()..].trim().to_string());
            } else if line.starts_with("Composition:") {
                error.composition = line["Composition:".len()..].trim().to_string();
            } else if line.starts_with("Layer:") {
                error.layer = line["Layer:".len()..].trim().to_string();
            } else if line.starts_with("Property:") {
                error.property = line["Property:".len()..].trim().to_string();
            } else if line.starts_with("Error:") {
                error.message = line["Error:".len()..].trim().to_string();
            }
        }

        if !error.message.is_empty() {
            errors.push(error);
        }
    }

    errors
}

pub fn audit_project_expressions(path: &str) -> ExpressionAuditResult {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return ExpressionAuditResult {
            success: false,
            project_path: path.to_string(),
            errors: Vec::new(),
            risky_count: 0,
        };
    }

    let is_aepx = path.to_lowercase().ends_with(".aepx");
    let errors = Vec::new();
    let mut risky_count = 0;

    if is_aepx {
        if let Ok(content) = fs::read_to_string(&path_buf) {
            // Very simple heuristic for static analysis in XML
            if content.contains("thisComp.layer") {
                // We'd ideally parse XML and find exact instances
                // For now, let's just flag the project if it has hardcoded layer refs
                risky_count += content.matches("thisComp.layer(\"").count();
            }
        }
    }

    ExpressionAuditResult {
        success: true,
        project_path: path.to_string(),
        errors,
        risky_count,
    }
}
