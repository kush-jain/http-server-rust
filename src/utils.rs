use std::io::Write;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;

#[cfg(test)]
use std::env;

#[cfg(test)]
pub fn get_project_root() -> Option<String> {
    let mut dir = env::current_dir().ok()?;
    while dir.parent().is_some() {
        if dir.join("Cargo.toml").exists() {
            return dir.to_str().map(|s| s.to_string());
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
}

#[cfg(test)]
pub fn get_project_source() -> Option<String> {
    let root = get_project_root()?;
    Some(format!("{}/src", root))
}

pub fn is_safe_path(path: &Path, base_dir: &Path) -> bool {
    // For existing files, use the file path
    if path.exists() {
        return path.canonicalize().map_or(false, |canon_path| {
            canon_path.starts_with(base_dir.canonicalize().unwrap_or_default())
        });
    }

    // For non-existent files, check the parent directory
    let parent = path.parent().unwrap_or(Path::new(""));
    parent.canonicalize().map_or(false, |canon_parent| {
        canon_parent.starts_with(base_dir.canonicalize().unwrap_or_default())
    })
}


pub fn gzip_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}