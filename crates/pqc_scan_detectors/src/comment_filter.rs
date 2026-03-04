use std::path::Path;

pub fn should_skip_comment_only_match(path: &Path, text: &str, start: usize) -> bool {
    let language = match detect_language(path) {
        Some(v) => v,
        None => return false,
    };
    should_skip_comment_only_match_for_language(language, text, start)
}

pub fn should_skip_comment_only_match_for_language(
    language: &str,
    text: &str,
    start: usize,
) -> bool {
    if language == "java" {
        return false;
    }

    let line = line_text_at_offset(text, start);
    is_comment_only_line(language, line)
}

fn detect_language(path: &Path) -> Option<&'static str> {
    let ext = path
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "java" => Some("java"),
        "js" | "mjs" | "cjs" | "jsx" => Some("javascript"),
        "ts" | "tsx" => Some("typescript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "rs" => Some("rust"),
        "rb" => Some("ruby"),
        _ => None,
    }
}

fn line_text_at_offset(text: &str, start: usize) -> &str {
    let safe_start = usize::min(start, text.len());
    let left = text[..safe_start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let right = text[safe_start..]
        .find('\n')
        .map(|i| safe_start + i)
        .unwrap_or(text.len());
    &text[left..right]
}

fn is_comment_only_line(language: &str, line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }

    match language {
        "javascript" | "typescript" | "go" | "rust" => {
            trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with("*/")
        }
        "python" => trimmed.starts_with('#'),
        "ruby" => {
            trimmed.starts_with('#') || trimmed.starts_with("=begin") || trimmed.starts_with("=end")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_js_comment_only_line() {
        let text = "// RS256 should not count\nconst a = 1;\n";
        let start = text.find("RS256").expect("contains RS256");
        assert!(should_skip_comment_only_match_for_language(
            "javascript",
            text,
            start
        ));
    }

    #[test]
    fn does_not_skip_js_inline_code_line() {
        let text = "const alg = \"RS256\"; // comment\n";
        let start = text.find("RS256").expect("contains RS256");
        assert!(!should_skip_comment_only_match_for_language(
            "javascript",
            text,
            start
        ));
    }

    #[test]
    fn does_not_skip_java_comment_match_by_policy() {
        let text = "// RS256 in java file\n";
        let start = text.find("RS256").expect("contains RS256");
        assert!(!should_skip_comment_only_match_for_language(
            "java", text, start
        ));
    }
}
