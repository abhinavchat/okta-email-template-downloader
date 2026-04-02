use std::path::{Path, PathBuf};

pub fn template_paths(
    destination: &Path,
    brand: &str,
    template: &str,
) -> (PathBuf, PathBuf, PathBuf) {
    let base = destination
        .join(sanitize_name(brand))
        .join(sanitize_name(template));
    let html = base.join("email.html");
    let subject = base.join("subject.txt");
    (base, html, subject)
}

pub fn sanitize_name(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        let valid = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_';
        if valid {
            result.push(ch);
        } else if !result.ends_with('_') {
            result.push('_');
        }
    }

    let trimmed = result.trim_matches('_');
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_name;

    #[test]
    fn sanitizes_invalid_characters() {
        assert_eq!(sanitize_name("Brand / Name"), "Brand_Name");
        assert_eq!(sanitize_name(""), "unnamed");
    }
}
