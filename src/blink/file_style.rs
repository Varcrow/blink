use crate::blink::entries::FileEntry;
use ratatui::style::Color;
#[cfg(unix)]
use std::path::Path;

pub fn get_file_icon(entry: &FileEntry) -> &'static str {
    // Check for directories first
    if entry.is_dir {
        // Special directory names
        return match entry.name.as_str() {
            ".git" => "\u{f1d3}",
            "node_modules" => "\u{e718}",
            ".github" => "\u{f408}",
            ".vscode" => "\u{e70c}",
            "target" => "\u{f484}",
            "build" | "dist" | "out" => "\u{f487}",
            "src" => "\u{f121}",
            "test" | "tests" => "\u{f0668}",
            "docs" => "\u{f02d}",
            "assets" | "public" => "\u{f03e}",
            "config" => "\u{e5fc}",
            _ => "\u{f07c}",
        };
    }

    // Check for specific filenames (no extension)
    match entry.name.as_str() {
        // Config files
        "Dockerfile" | "dockerfile" => return "\u{f308}",
        "docker-compose.yml" | "docker-compose.yaml" => return "\u{f308}",
        "Makefile" | "makefile" => return "\u{f489}",
        "CMakeLists.txt" => return "\u{f489}",
        "Cargo.toml" | "Cargo.lock" => return "\u{e7a8}",
        "package.json" | "package-lock.json" => return "\u{e718}",
        "requirements.txt" | "pyproject.toml" | "setup.py" => return "\u{e73c}",
        "go.mod" | "go.sum" => return "\u{e626}",
        "Gemfile" | "Gemfile.lock" => return "\u{e21e}",

        // Git files
        ".gitignore" | ".gitattributes" | ".gitmodules" => return "\u{f1d3}",
        ".gitconfig" => return "\u{f1d3}",

        // CI/CD
        ".travis.yml" | ".gitlab-ci.yml" | "azure-pipelines.yml" => return "\u{f144}",
        "Jenkinsfile" => return "\u{e767}",

        // Editor configs
        ".editorconfig" => return "\u{e615}",
        ".eslintrc" | ".eslintrc.js" | ".eslintrc.json" => return "\u{e60c}",
        ".prettierrc" | ".prettierrc.js" | ".prettierrc.json" => return "\u{e60b}",

        // Documentation
        "README.md" | "README" | "readme.md" => return "\u{f48a}",
        "LICENSE" | "LICENSE.md" | "COPYING" => return "\u{f0219}",
        "CHANGELOG.md" | "CHANGELOG" => return "\u{f0753}",

        // Shell configs
        ".bashrc" | ".bash_profile" | ".zshrc" | ".zsh_profile" => return "\u{f489}",
        ".profile" => return "\u{f489}",

        _ => {}
    }

    // Get file extension
    let extension = entry
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        // Programming Languages
        "rs" => "\u{e7a8}",
        "py" | "pyw" | "pyc" | "pyd" | "pyo" => "\u{e73c}",
        "js" | "mjs" | "cjs" => "\u{e74e}",
        "jsx" => "\u{e7ba}",
        "ts" => "\u{e628}",
        "tsx" => "\u{e7ba}",
        "go" => "\u{e626}",
        "c" => "\u{e61e}",
        "cpp" | "cc" | "cxx" | "c++" => "\u{e61d}",
        "h" | "hpp" | "hxx" | "h++" => "\u{e61e}",
        "java" => "\u{e738}",
        "class" | "jar" => "\u{e738}",
        "rb" | "erb" => "\u{e21e}",
        "php" => "\u{e73d}",
        "swift" => "\u{e755}",
        "kt" | "kts" => "\u{e634}",
        "scala" | "sc" => "\u{e737}",
        "cs" => "\u{f81a}",
        "fs" | "fsx" | "fsi" => "\u{e7a7}",
        "clj" | "cljs" | "cljc" => "\u{e76a}",
        "ex" | "exs" => "\u{e62d}",
        "erl" | "hrl" => "\u{e7b1}",
        "hs" | "lhs" => "\u{e777}",
        "lua" => "\u{e620}",
        "perl" | "pl" | "pm" => "\u{e769}",
        "r" => "\u{f25d}",
        "dart" => "\u{e798}",
        "vim" => "\u{e62b}",
        "lisp" | "el" => "\u{f671}",

        // Web Technologies
        "html" | "htm" => "\u{e736}",
        "css" => "\u{e749}",
        "scss" | "sass" => "\u{e749}",
        "less" => "\u{e758}",
        "vue" => "\u{f0844}",
        "svelte" => "\u{e697}",
        "astro" => "\u{e6b2}",

        // Data & Config
        "json" | "jsonc" => "\u{e60b}",
        "yaml" | "yml" => "\u{f481}",
        "toml" => "\u{e615}",
        "xml" => "\u{e619}",
        "csv" => "\u{f1c0}",
        "sql" => "\u{e706}",
        "db" | "sqlite" | "sqlite3" => "\u{e706}",
        "ini" | "cfg" | "conf" | "config" => "\u{e615}",
        "env" => "\u{f462}",

        // Documents
        "md" | "markdown" | "mdown" | "mkd" => "\u{e609}",
        "txt" | "text" => "\u{f15c}",
        "pdf" => "\u{f1c1}",
        "doc" | "docx" => "\u{f1c2}",
        "xls" | "xlsx" => "\u{f1c3}",
        "ppt" | "pptx" => "\u{f1c4}",
        "rtf" => "\u{f15c}",
        "tex" | "latex" => "\u{e600}",
        "org" => "\u{e633}",
        // Images
        "png" => "\u{f1c5}",
        "jpg" | "jpeg" => "\u{f1c5}",
        "gif" => "\u{f1c5}",
        "svg" => "\u{f1c5}",
        "ico" | "icon" => "\u{f1c5}",
        "bmp" => "\u{f1c5}",
        "webp" => "\u{f1c5}",
        "tiff" | "tif" => "\u{f1c5}",
        "psd" => "\u{e7b8}",
        "ai" => "\u{e7b4}",
        "sketch" => "\u{e6c2}",
        "fig" => "\u{e6c6}",

        // Video
        "mp4" | "m4v" => "\u{f03d}",
        "avi" => "\u{f03d}",
        "mkv" => "\u{f03d}",
        "mov" => "\u{f03d}",
        "wmv" => "\u{f03d}",
        "flv" => "\u{f03d}",
        "webm" => "\u{f03d}",

        // Audio
        "mp3" => "\u{f001}",
        "wav" => "\u{f001}",
        "flac" => "\u{f001}",
        "ogg" => "\u{f001}",
        "m4a" => "\u{f001}",
        "aac" => "\u{f001}",
        "wma" => "\u{f001}",

        // Archives
        "zip" => "\u{f410}",
        "tar" => "\u{f410}",
        "gz" | "gzip" => "\u{f410}",
        "bz2" | "bzip2" => "\u{f410}",
        "xz" => "\u{f410}",
        "rar" => "\u{f410}",
        "7z" => "\u{f410}",
        "iso" => "\u{e271}",
        "dmg" => "\u{e271}",

        // Executables & Binaries
        "exe" | "msi" => "\u{f17a}",
        "dll" | "so" | "dylib" => "\u{f17c}",
        "app" => "\u{f179}",
        "deb" => "\u{f187}",
        "rpm" => "\u{f187}",
        "apk" => "\u{e70e}",

        // Shell Scripts
        "sh" | "bash" | "zsh" | "fish" => "\u{f489}",
        "bat" | "cmd" | "ps1" => "\u{f489}",

        // Build & Package
        "lock" => "\u{f023}",
        "log" => "\u{f18d}",
        "tmp" | "temp" => "\u{f2ed}",
        "bak" | "backup" => "\u{f0a0}",
        "swp" | "swo" => "\u{e62b}",
        "cache" => "\u{f49b}",

        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "\u{f031}",

        // 3D Models
        "obj" | "fbx" | "dae" | "3ds" | "blend" => "\u{e79b}",

        // Default
        _ => "\u{f15b}",
    }
}

/// Get the color for a file or directory
pub fn get_file_color(entry: &FileEntry) -> Color {
    // Directories
    if entry.is_dir {
        return match entry.name.as_str() {
            ".git" => Color::Red,
            "node_modules" => Color::Rgb(139, 195, 74),
            ".github" => Color::Rgb(88, 96, 105),
            ".vscode" => Color::Rgb(0, 122, 204),
            "target" | "build" | "dist" | "out" => Color::Rgb(255, 152, 0),
            "src" => Color::Rgb(33, 150, 243),
            "test" | "tests" => Color::Rgb(255, 235, 59),
            "docs" => Color::Rgb(76, 175, 80),
            "assets" | "public" => Color::Rgb(156, 39, 176),
            "config" => Color::Rgb(96, 125, 139),
            _ => Color::Rgb(100, 181, 246),
        };
    }

    // Special files by name
    match entry.name.as_str() {
        "Dockerfile" | "docker-compose.yml" | "docker-compose.yaml" => {
            return Color::Rgb(0, 184, 212);
        }
        "Makefile" | "makefile" | "CMakeLists.txt" => return Color::Rgb(117, 117, 117),
        "Cargo.toml" | "Cargo.lock" => return Color::Rgb(222, 165, 132),
        "package.json" | "package-lock.json" => return Color::Rgb(203, 56, 55),
        "go.mod" | "go.sum" => return Color::Rgb(0, 173, 216),
        ".gitignore" | ".gitattributes" | ".gitmodules" | ".gitconfig" => return Color::Red,
        "README.md" | "README" | "readme.md" => return Color::Rgb(65, 185, 131),
        "LICENSE" | "LICENSE.md" | "COPYING" => return Color::Rgb(255, 193, 7),
        _ => {}
    }

    let extension = entry
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "rs" => Color::Rgb(222, 165, 132),
        "py" | "pyw" | "pyc" | "pyd" | "pyo" => Color::Rgb(53, 114, 165),
        "js" | "mjs" | "cjs" => Color::Rgb(240, 219, 79),
        "jsx" => Color::Rgb(97, 218, 251),
        "ts" | "tsx" => Color::Rgb(49, 120, 198),
        "go" => Color::Rgb(0, 173, 216),
        "c" | "h" => Color::Rgb(85, 85, 85),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Color::Rgb(0, 89, 157),
        "java" | "class" | "jar" => Color::Rgb(244, 67, 54),
        "rb" | "erb" => Color::Rgb(204, 52, 45),
        "php" => Color::Rgb(119, 123, 180),
        "swift" => Color::Rgb(255, 69, 58),
        "kt" | "kts" => Color::Rgb(127, 82, 236),
        "cs" => Color::Rgb(104, 33, 122),
        "dart" => Color::Rgb(0, 180, 240),
        "lua" => Color::Rgb(0, 0, 128),
        "html" | "htm" => Color::Rgb(227, 76, 38),
        "css" => Color::Rgb(38, 77, 228),
        "scss" | "sass" => Color::Rgb(207, 100, 154),
        "vue" => Color::Rgb(65, 184, 131),
        "svelte" => Color::Rgb(255, 62, 0),
        "json" | "jsonc" => Color::Rgb(251, 193, 60),
        "yaml" | "yml" => Color::Rgb(204, 51, 0),
        "toml" => Color::Rgb(156, 163, 175),
        "xml" => Color::Rgb(230, 126, 34),
        "sql" | "db" | "sqlite" | "sqlite3" => Color::Rgb(236, 107, 86),
        "ini" | "cfg" | "conf" | "config" | "env" => Color::Rgb(117, 117, 117),
        "md" | "markdown" | "mdown" | "mkd" => Color::Rgb(66, 165, 245),
        "txt" | "text" => Color::Rgb(189, 189, 189),
        "pdf" => Color::Rgb(211, 47, 47),
        "doc" | "docx" => Color::Rgb(33, 150, 243),
        "xls" | "xlsx" => Color::Rgb(67, 160, 71),
        "ppt" | "pptx" => Color::Rgb(255, 87, 34),
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" | "tif" => {
            Color::Rgb(171, 71, 188)
        }
        "svg" => Color::Rgb(255, 181, 71),
        "ico" | "icon" => Color::Rgb(244, 143, 177),
        "psd" | "ai" | "sketch" | "fig" => Color::Rgb(49, 168, 255),
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => Color::Rgb(255, 82, 82),
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma" => Color::Rgb(156, 39, 176),
        "zip" | "tar" | "gz" | "gzip" | "bz2" | "xz" | "rar" | "7z" | "iso" | "dmg" => {
            Color::Rgb(239, 83, 80)
        }
        "exe" | "msi" | "dll" | "so" | "dylib" | "app" | "deb" | "rpm" | "apk" => {
            Color::Rgb(76, 175, 80)
        }
        "sh" | "bash" | "zsh" | "fish" | "bat" | "cmd" | "ps1" => Color::Rgb(77, 182, 172),
        "lock" => Color::Rgb(255, 193, 7),
        "log" => Color::Rgb(158, 158, 158),
        "tmp" | "temp" | "cache" => Color::Rgb(117, 117, 117),
        "bak" | "backup" | "swp" | "swo" => Color::Rgb(96, 125, 139),
        "ttf" | "otf" | "woff" | "woff2" | "eot" => Color::Rgb(255, 235, 59),
        _ => Color::Rgb(189, 189, 189),
    }
}

/// Helper to check if a file is executable (Unix systems)
#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn get_file_icon_enhanced(entry: &FileEntry) -> &'static str {
    #[cfg(unix)]
    if !entry.is_dir && is_executable(&entry.path) {
        return "\u{f489}"; // 
    }

    get_file_icon(entry)
}

pub fn get_file_color_enhanced(entry: &FileEntry) -> Color {
    #[cfg(unix)]
    if !entry.is_dir && is_executable(&entry.path) {
        return Color::Rgb(76, 175, 80);
    }

    get_file_color(entry)
}
