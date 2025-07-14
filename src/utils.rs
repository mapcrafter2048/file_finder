use chrono::{DateTime, Local};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if size == 0 {
        return "0 B".to_string();
    }

    let mut size_f = size as f64;
    let mut unit_index = 0;

    while size_f >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size_f /= THRESHOLD as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}

pub fn format_modified_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_file_icon(path: &Path) -> String {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        // Programming languages
        "rs" => "🦀".to_string(),
        "py" => "🐍".to_string(),
        "js" | "mjs" => "🟨".to_string(),
        "ts" => "🔷".to_string(),
        "java" => "☕".to_string(),
        "cpp" | "cc" | "cxx" => "⚙️".to_string(),
        "c" => "🔧".to_string(),
        "cs" => "🔵".to_string(),
        "go" => "🐹".to_string(),
        "php" => "🐘".to_string(),
        "rb" => "💎".to_string(),
        "swift" => "🐦".to_string(),
        "kt" => "🟠".to_string(),
        "dart" => "🎯".to_string(),
        "scala" => "⚡".to_string(),
        "clj" | "cljs" => "🎭".to_string(),
        "hs" => "🎪".to_string(),
        "ml" | "mli" => "🐪".to_string(),
        "r" => "📊".to_string(),
        "jl" => "🔮".to_string(),
        
        // Web technologies
        "html" | "htm" => "🌐".to_string(),
        "css" => "🎨".to_string(),
        "scss" | "sass" => "💅".to_string(),
        "less" => "📝".to_string(),
        "vue" => "💚".to_string(),
        "jsx" | "tsx" => "⚛️".to_string(),
        "svelte" => "🔥".to_string(),
        
        // Data formats
        "json" => "📋".to_string(),
        "xml" => "📄".to_string(),
        "yaml" | "yml" => "⚙️".to_string(),
        "toml" => "🔧".to_string(),
        "csv" => "📊".to_string(),
        "sql" => "🗃️".to_string(),
        
        // Documents
        "md" | "markdown" => "📖".to_string(),
        "txt" => "📝".to_string(),
        "pdf" => "📕".to_string(),
        "doc" | "docx" => "📘".to_string(),
        "xls" | "xlsx" => "📗".to_string(),
        "ppt" | "pptx" => "📙".to_string(),
        
        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => "🖼️".to_string(),
        "ico" => "🎯".to_string(),
        
        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "📦".to_string(),
        
        // Executables
        "exe" | "app" | "deb" | "rpm" | "dmg" | "msi" => "⚡".to_string(),
        
        // Config files
        "cfg" | "conf" | "ini" | "env" => "⚙️".to_string(),
        "gitignore" | "gitattributes" => "🔀".to_string(),
        "dockerfile" => "🐳".to_string(),
        
        // Lock files
        "lock" => "🔒".to_string(),
        
        // Logs
        "log" => "📜".to_string(),
        
        // Shell scripts
        "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" => "💻".to_string(),
        
        // Default
        _ => "📄".to_string(),
    }
}

pub fn is_binary_file(path: &Path) -> bool {
    // Common binary file extensions
    let binary_extensions = [
        "exe", "dll", "so", "dylib", "bin", "o", "obj",
        "jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "webp", "svg",
        "mp3", "mp4", "avi", "mkv", "mov", "wmv", "flv", "wav", "flac",
        "zip", "rar", "7z", "tar", "gz", "bz2", "xz",
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "ttf", "otf", "woff", "woff2",
    ];

    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        return binary_extensions.contains(&ext.to_lowercase().as_str());
    }

    // Try to read first few bytes to detect binary
    if let Ok(bytes) = fs::read(path) {
        if bytes.len() > 0 {
            let sample_size = std::cmp::min(bytes.len(), 512);
            let null_count = bytes[..sample_size].iter().filter(|&&b| b == 0).count();
            // If more than 1% of the sample contains null bytes, consider it binary
            return null_count as f64 / sample_size as f64 > 0.01;
        }
    }

    false
}

pub fn should_search_file(path: &Path, extensions: &Option<HashSet<String>>) -> bool {
    if let Some(ref allowed_extensions) = extensions {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            return allowed_extensions.contains(&ext.to_lowercase());
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(500), "500 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_get_file_icon() {
        assert_eq!(get_file_icon(Path::new("test.rs")), "🦀");
        assert_eq!(get_file_icon(Path::new("test.py")), "🐍");
        assert_eq!(get_file_icon(Path::new("test.js")), "🟨");
        assert_eq!(get_file_icon(Path::new("test.unknown")), "📄");
    }
}
