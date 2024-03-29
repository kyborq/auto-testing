use std::fs::DirEntry;
use std::path::Path;
use std::{fs, io};

pub fn get_test_files(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut test_files = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("txt")
                && path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.starts_with("test"))
                    .unwrap_or(false)
            {
                test_files.push(entry);
            }
        }
    }
    Ok(test_files)
}
