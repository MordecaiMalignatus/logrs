use std::env;
use std::path;
use std::io::Error;
use std::process::exit; 
use std::path::PathBuf;

use io::read_file;
use io::write_to_file;

extern crate toml; 

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub base_filepath: path::PathBuf,
}

pub fn get_config() -> Config { 
    let mut dotfile_path = env::home_dir().unwrap();
    dotfile_path.push(".logrs");

    match read_file(dotfile_path) {
        Ok(file) => {
            match toml::from_str(&file) {
                Ok(c) => c, 
                Err(_) => {
                    println!("Can't decode dotfile, fix the syntax or delete the file, please. <3");
                    exit(1)
                }
            } 
        }, 
        Err(_) => { 
            match make_default_dotfile() {
                Ok(t) => t,
                Err(e) => {
                    panic!("Can't create default dotfile, reason: {}", e);
                }
            }
        }
    }
}

fn default_config() -> Config {
    let logs_path = match env::home_dir() {
        Some(mut path) => {
            // Add "/logs/" to user home directory path."
            path.push("logs");
            path
        }
        None => panic!("Unable to get your home dir!"),
    };

    Config { base_filepath: logs_path }
}

// This needs to be reworked to actuall use PathBuf and find the dotfile with that.

fn make_default_dotfile() -> Result<Config, Error> {
    let config = default_config();
    // We can assume a hand-generated config is serializable...
    let config_string = toml::to_string(&config).expect("Cannot turn default Config to string");
    let dotfile = &dotfile_path();

    match write_to_file(dotfile, config_string) {
        Ok(_) => {
            println!("Creating dotfile...");
            Ok(config)
        }, 
        Err(e) => Err(e),
    }
}

fn dotfile_path() -> PathBuf {
    let mut f = env::home_dir().unwrap();
    f.push(".logrs");
    f
}

#[cfg(test)]
mod test {

    #[test]
    fn test_dotfile_existance_invariant() {

    }
}
