use std::fs::read_dir;
use std::path::Path;

use config::Config;

// List all days with record.
pub fn days(config: &Config) {
    let dir = Path::new(&config.base_filepath);
    match read_dir(dir) {
        Ok(dir_iter) => {
            for dir_item in dir_iter {
                let entry = match dir_item {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Unable to read directory entry: {}", e);
                        continue;
                    }
                };
                let path = entry.path();
                if has_records(&path) {
                    println!("{}", entry.file_name().into_string().unwrap());
                }
            }
        },
        Err(e) => eprintln!("Error reading dir: {}, error: {}", dir.display(), e),
    }
}

fn has_records(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    match read_dir(path) {
        Ok(dir_iter) => {
            if dir_iter.count() > 0 {
                return true;
            }
        },
        Err(e) => {
            eprintln!("Error reading dir: {}, error: {}", path.display(), e);
            return false;
        }
    }
    return false;
}