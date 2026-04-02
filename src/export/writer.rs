use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::export::format_html::normalize_html;
use crate::export::paths::template_paths;

#[derive(Debug, Clone)]
pub struct ExportOutcome {
    pub template_name: String,
    pub html_path: PathBuf,
    pub subject_path: PathBuf,
}

pub struct ExportPaths {
    pub base: PathBuf,
    pub html_path: PathBuf,
    pub subject_path: PathBuf,
}

pub fn export_paths(destination: &Path, brand_name: &str, template_name: &str) -> ExportPaths {
    let (base, html_path, subject_path) = template_paths(destination, brand_name, template_name);
    ExportPaths {
        base,
        html_path,
        subject_path,
    }
}

pub fn export_exists(destination: &Path, brand_name: &str, template_name: &str) -> bool {
    let paths = export_paths(destination, brand_name, template_name);
    paths.html_path.exists() || paths.subject_path.exists()
}

pub fn export_template(
    destination: &Path,
    brand_name: &str,
    template_name: &str,
    subject: &str,
    html_body: &str,
    overwrite: bool,
) -> AppResult<ExportOutcome> {
    ensure_destination(destination)?;

    let paths = export_paths(destination, brand_name, template_name);
    fs::create_dir_all(&paths.base)?;

    if !overwrite && (paths.html_path.exists() || paths.subject_path.exists()) {
        return Err(AppError::Config {
            message: format!(
                "files for template '{template_name}' already exist. Re-run with --overwrite to replace them"
            ),
        });
    }

    fs::write(&paths.html_path, normalize_html(html_body))?;
    fs::write(&paths.subject_path, normalize_subject(subject))?;

    Ok(ExportOutcome {
        template_name: template_name.to_string(),
        html_path: paths.html_path,
        subject_path: paths.subject_path,
    })
}

fn ensure_destination(destination: &Path) -> AppResult<()> {
    if destination.exists() {
        if !destination.is_dir() {
            return Err(AppError::PathNotDirectory(destination.to_path_buf()));
        }
    } else {
        fs::create_dir_all(destination)?;
    }

    Ok(())
}

fn normalize_subject(subject: &str) -> String {
    let trimmed = subject.trim();
    format!("{trimmed}\n")
}

#[cfg(test)]
mod tests {
    use super::normalize_subject;

    #[test]
    fn normalizes_subject_with_trailing_newline() {
        assert_eq!(normalize_subject(" Hello "), "Hello\n");
    }
}
