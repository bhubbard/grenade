//! Lints HTML files against the mechanical rules in `SKILL.md`:
//! the 5 allowed font sizes, no emoji, no `white-space: nowrap`, no gradients.

use std::process::ExitCode;
use std::{env, fs};

/// Font sizes (px) the style kit allows.
const ALLOWED_SIZES: [f64; 5] = [40.0, 32.0, 22.0, 16.0, 13.0];

/// A single rule violation.
#[derive(Debug, PartialEq)]
struct Finding {
    line: usize,
    message: String,
}

fn is_emoji(c: char) -> bool {
    matches!(c as u32,
        0x1F300..=0x1FAFF | 0x2600..=0x27BF | 0x1F000..=0x1F2FF | 0xFE0F)
}

/// Returns the px values of every `font-size: Npx` declaration in `line`.
fn font_sizes_px(line: &str) -> Vec<f64> {
    let mut sizes = Vec::new();
    let mut rest = line;
    while let Some(i) = rest.find("font-size") {
        rest = &rest[i + "font-size".len()..];
        let value: String = rest
            .trim_start_matches(|c: char| c == ':' || c.is_whitespace())
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let after = rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace());
        if after[value.len()..].starts_with("px") {
            if let Ok(n) = value.parse() {
                sizes.push(n);
            }
        }
    }
    sizes
}

fn lint(html: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (idx, line) in html.lines().enumerate() {
        let line_no = idx + 1;
        let mut add = |message: String| findings.push(Finding { line: line_no, message });
        if let Some(c) = line.chars().find(|&c| is_emoji(c)) {
            add(format!("emoji {c:?}: use a line icon in SVG"));
        }
        if line.replace(' ', "").contains("white-space:nowrap") {
            add("white-space: nowrap: boxes must grow with content".into());
        }
        if line.contains("gradient(") {
            add("gradient: the style kit has none".into());
        }
        for size in font_sizes_px(line) {
            if !ALLOWED_SIZES.contains(&size) {
                add(format!("font-size {size}px: allowed sizes are 40, 32, 22, 16, 13"));
            }
        }
    }
    findings
}

fn main() -> ExitCode {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: noslop-lint <file.html>...");
        return ExitCode::from(2);
    }
    let mut failed = false;
    for path in &paths {
        match fs::read_to_string(path) {
            Ok(html) => {
                for f in lint(&html) {
                    println!("{path}:{}: {}", f.line, f.message);
                    failed = true;
                }
            }
            Err(e) => {
                eprintln!("{path}: {e}");
                failed = true;
            }
        }
    }
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_page_passes() {
        assert!(lint("<p style=\"font-size:16px\">hi</p>").is_empty());
    }

    #[test]
    fn flags_bad_size_nowrap_gradient_emoji() {
        let html = "a{font-size: 18px}\nb{white-space: nowrap}\nc{background:linear-gradient(red,blue)}\n🚀";
        assert_eq!(lint(html).len(), 4);
    }
}
