use io::read_file;
use std::io::Error;

pub struct Config {
    pub base_filepath: String,
}

fn default_config() -> Config {
    Config {
        base_filepath: "/Users/az/logs/".to_owned(),
    }
}

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