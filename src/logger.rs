use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;
use config::Config;

extern crate chrono;
use self::chrono::*;

pub fn log(entry: String, config: &Config) -> Result<(), io::Error> {
    let now = Local::now();
    let formatted_log = format_log_entry(entry, now);
    let filename = get_file_name(now.date());
    let filepath = Path::new(&config.base_filepath).join(filename);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)
        .unwrap();

    match file.write(formatted_log.as_bytes()) {
        Ok(_) => {
            println!("Noted.");
            Ok(())
        },
        Err(x) => Err(x),
    }
}

fn format_log_entry(entry: String, time: DateTime<Local>) -> String {
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

#[cfg(test)]
mod test{
    use super::chrono::*;

    #[test]
    fn file_name_correctness_test() {
        let time = Local.ymd(2000, 1, 1);
        assert_eq!(super::get_file_name(time), "2000-01-01");

        let time_second = Local.ymd(2011, 11, 2);
        assert_eq!(super::get_file_name(time_second), "2011-11-02");
    }
}