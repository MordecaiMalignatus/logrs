use io::read_file;
use std::io::Error;
use std::env;
use std::path;

pub struct Config {
    pub base_filepath: path::PathBuf,
}

pub fn default_config() -> Config {
    let logs_path = match env::home_dir() {
        Some(mut path) => {
            // Add "/logs/" to user home directory path."
            path.push("logs");
            path
        },
        None => panic!("Unable to get your home dir!"),
    };

    Config {
        base_filepath: logs_path,
    }
}

// This needs to be reworked to actuall use PathBuf and find the dotfile with that.

// pub fn read_from_dotfile() -> Config {
//     match read_file("~/.logrs") {
//         Ok(file_content) => {
//             default_config()
//         }
//         Err(e) => {
//             println!("Couldn't find dotfile, creating...");
//             make_default_dotfile().expect("Can't create default dotfile. Shit's fucked.")
//         }
//     }
// }

fn make_default_dotfile() -> Result<Config, Error> {
    Ok(default_config())
    // TODO Write that to disk as well.
}