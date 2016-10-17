use config::Config;
use logger::log;
use logger::get_file_name;
use io::print_file;

use std::io;

extern crate chrono;
use self::chrono::*;

pub fn dispatch(entry: String, config: &Config) -> Result<(), io::Error> {
    if entry.starts_with("show") {
        dispatch_display(entry, config)
    } else {
        log(entry, config)
    }
}

fn dispatch_display(entry: String, config: &Config) -> Result<(), io::Error> {
    let now = Local::now();

    if entry.starts_with("show today") {
        let file_name = get_file_name(now.date());
        let mut path = config.base_filepath.clone();
        path.push_str(&file_name);

        print_file(path)

    } else if entry.starts_with("show yesterday") {

        let today = now.date();
        let yesterday = today - Duration::days(1);
        let file_name = get_file_name(yesterday);

        let mut path = config.base_filepath.clone();
        path.push_str(&file_name);

        print_file(path)
    } else {
        Ok(())
    }
}