//! `.lessonbinderignore` at a course root (gitignore-style patterns).

use std::path::Path;

const IGNORE_FILE: &str = ".lessonbinderignore";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LessonbinderIgnore {
    patterns: Vec<String>,
}

impl LessonbinderIgnore {
    pub fn load_from_course_root(root: &Path) -> std::io::Result<Self> {
        let path = root.join(IGNORE_FILE);
        if !path.is_file() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)?;
        Ok(Self::parse_ignore_text(&text))
    }

    pub fn parse_ignore_text(content: &str) -> Self {
        Self {
            patterns: parse_patterns(content),
        }
    }

    pub fn is_ignored(&self, relative_path: &str) -> bool {
        let relative_path = relative_path.trim_start_matches("./");
        self.patterns
            .iter()
            .any(|pattern| path_matches_pattern(relative_path, pattern))
    }
}

pub fn parse_patterns(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.split('#').next().unwrap_or("").trim())
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}

fn path_matches_pattern(relative_path: &str, pattern: &str) -> bool {
    let pattern = pattern.trim();
    if pattern.is_empty() {
        return false;
    }
    let relative_path = relative_path.replace('\\', "/");
    let normalized_pattern = pattern.replace('\\', "/");

    if normalized_pattern.contains('/') {
        let anchored = normalized_pattern
            .strip_prefix('/')
            .unwrap_or(&normalized_pattern);
        return match_path_segments(&relative_path, anchored);
    }

    let file_name = relative_path
        .rsplit('/')
        .next()
        .unwrap_or(relative_path.as_str());
    if glob_segment(file_name, &normalized_pattern) {
        return true;
    }
    relative_path
        .split('/')
        .any(|segment| glob_segment(segment, &normalized_pattern))
}

fn match_path_segments(relative_path: &str, pattern: &str) -> bool {
    let path_segments: Vec<&str> = relative_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    let pattern_segments: Vec<&str> = pattern
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    match_segments(&path_segments, &pattern_segments)
}

fn match_segments(path: &[&str], pattern: &[&str]) -> bool {
    if pattern.is_empty() {
        return path.is_empty();
    }
    if path.is_empty() {
        return false;
    }
    if pattern[0] == "*" {
        return match_segments(&path[1..], &pattern[1..]);
    }
    if !glob_segment(path[0], pattern[0]) {
        return false;
    }
    match_segments(&path[1..], &pattern[1..])
}

fn glob_segment(text: &str, pattern: &str) -> bool {
    match pattern.split_once('*') {
        None => text == pattern,
        Some(("", suffix)) => text.ends_with(suffix),
        Some((prefix, "")) => text.starts_with(prefix),
        Some((prefix, suffix)) => {
            text.starts_with(prefix)
                && text.ends_with(suffix)
                && text.len() >= prefix.len() + suffix.len()
        }
    }
}
