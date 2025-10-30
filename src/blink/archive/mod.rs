use std::path::{Path, PathBuf};

pub mod zip;

enum CompressionFormat {
    Gzip,
    Bzip2,
    Zstd,
    Zip,
}

impl CompressionFormat {
    fn extension(&self) -> &str {
        match self {
            Self::Gzip => "gz",
            Self::Bzip2 => "bz2",
            Self::Zstd => "zst",
            Self::Zip => "zip",
        }
    }

    fn default() -> Self {
        Self::Gzip // load from config
    }
}

fn detect_format(path: &Path) -> Option<CompressionFormat> {
    match path.extension()?.to_str()? {
        "gz" => Some(CompressionFormat::Gzip),
        "bz2" => Some(CompressionFormat::Bzip2),
        "zst" => Some(CompressionFormat::Zstd),
        "zip" => Some(CompressionFormat::Zip),
        _ => None,
    }
}

fn is_compressed(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("gz" | "bz2" | "zst" | "zip")
    )
}

fn generate_compressed_path(input: &Path, format: Option<CompressionFormat>) -> PathBuf {
    let format = format.unwrap_or_else(CompressionFormat::default);
    let mut output = input.to_path_buf();

    // Add compression extension to existing extension
    let new_ext = match input.extension() {
        Some(ext) => format!("{}.{}", ext.to_str().unwrap(), format.extension()),
        None => format.extension().to_string(),
    };

    output.set_extension(new_ext);
    output
}
