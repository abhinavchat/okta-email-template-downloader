pub fn normalize_html(input: &str) -> String {
    let mut lines = Vec::new();

    for line in input.replace("\r\n", "\n").replace('\r', "\n").lines() {
        lines.push(line.trim_end().to_string());
    }

    let mut output = lines.join("\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::normalize_html;

    #[test]
    fn normalizes_line_endings_and_trailing_whitespace() {
        let normalized = normalize_html("<html> \r\n<body>\t\r\n</body>\r\n</html>");
        assert_eq!(normalized, "<html>\n<body>\n</body>\n</html>\n");
    }
}
