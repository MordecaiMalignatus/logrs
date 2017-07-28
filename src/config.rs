use std::fs::create_dir;
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub base_filepath: PathBuf,
}

// TODO: refactor this part. Parts of code checking for directory and creating it should be in a different function.
pub fn default_config() -> Result<Config, String> {
    let logs_path = match env::home_dir() {
        Some(mut path) => {
            // Add "/logs/" to user home directory path."
            path.push("logs");
            path
        },
        None => return Err(String::from("Unable to get your home dir!")),
    };
    // Check if `logs_path` dir exists, and check if it is a directory.
    // We can't check if directory is writable on non-unix systems, so we skip it for now.
    if logs_path.exists() {
        match logs_path.metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    return Err(format!("File at path: {} is not a directory! Please remove it or specify different log path.", logs_path.display()))
                }
            }
            // If reading metadata fails, quit.
            Err(err) => {return Err(format!("Error reading metadata for directory: {}, error: {}", logs_path.display(), err))}
        }
    // If directory does not exists, try to create it.
    } else {
        match create_dir(logs_path.as_path()) {
            Ok(()) => {}
            Err(err) => {return Err(format!("Unable to create logs directory: {}, error: {}", logs_path.display(), err))}
        }
    }

    return Ok(Config {
        base_filepath: logs_path,
    });
}

/*  Not used for now.
pub fn read_from_dotfile() -> Config {
    match read_file("~/.logrs") {
        Ok(file_content) => {
            default_config()
        }
        Err(e) => {
            println!("Couldn't find dotfile, creating...");
            make_default_dotfile().expect("Can't create default dotfile. Shit's fucked.")
        }
    }
}

fn make_default_dotfile() -> Result<Config, Error> {
    Ok(default_config())
    // TODO Write that to disk as well.
}
*/