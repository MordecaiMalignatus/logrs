use std::io;
use std::path::Path;
use config::Config;
use io::append_to_file;

extern crate chrono;
use self::chrono::*;

pub fn log(entry: &String, config: &Config) -> Result<(), io::Error> {
    let now = Local::now();
    let formatted_log = format_log_entry(&entry, now);
    let filename = get_file_name(now.date());
    let filepath = Path::new(&config.base_filepath).join(filename);

    match append_to_file(filepath, formatted_log) {
        Ok(_)  => {
            println!("Noted.");
            Ok(())
        },
        Err(x) => Err(x),
    }
}

fn format_log_entry(entry: &String, time: DateTime<Local>) -> String {
    let timestamp = time.format("%H:%M").to_string();

    let mut res = String::new();
    res.push_str(" ");
    res.push_str(&timestamp);
    res.push_str(" - ");
    res.push_str(&entry);
    res.push_str("\n");

    res
}

pub fn get_file_name(time_of_entry: Date<Local>) -> String {
    time_of_entry.format("%Y-%m-%d").to_string()
}