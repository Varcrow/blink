use std::{
    fs::File,
    io::{self, copy},
    path::PathBuf,
};
use zip::CompressionMethod;
use zip::write::FileOptions;

pub fn create_zip_with_structure(input_paths: Vec<PathBuf>, output_name: &str) -> io::Result<()> {
    let file = File::create(output_name)?;
    let mut zip = zip::ZipWriter::new(file);

    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    for path in input_paths {
        if path.is_file() {
            let archive_path = path.to_str().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Invalid path encoding")
            })?;

            zip.start_file(archive_path, options)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

            let mut input = File::open(&path)?;
            copy(&mut input, &mut zip)?;
        } else if path.is_dir() {
            add_directory_to_zip(&mut zip, &path, &path)?;
        }
    }

    zip.finish()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
    Ok(())
}
fn add_directory_to_zip(
    zip: &mut zip::ZipWriter<File>,
    dir: &PathBuf,
    base: &PathBuf,
) -> io::Result<()> {
    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(base)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;

        if path.is_file() {
            let name = relative
                .to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?;

            zip.start_file(name, options)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

            let mut file = File::open(&path)?;
            copy(&mut file, zip)?;
        } else if path.is_dir() {
            add_directory_to_zip(zip, &path, base)?;
        }
    }
    Ok(())
}
