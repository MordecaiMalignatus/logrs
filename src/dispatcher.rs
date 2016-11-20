use config::Config;
use logger;
use logger::get_file_name;
use io::print_file;

use std::io;
use std::process::Command;

extern crate chrono;
use self::chrono::*;

pub fn dispatch(entry: String, config: &Config) -> Result<(), io::Error> {
    match entry {
        ref c if c.starts_with("show") => dispatch_display(&entry, config),
        ref c if c.starts_with("search") => dispatch_search(&entry, config),
        _ => logger::log(&entry, config)
    }
}

fn dispatch_search(entry: &str, config: &Config) -> Result<(), io::Error> {
    let search_string: &str = &entry[6..];

    let output = Command::new("grep")
        .arg("-ri")
        .arg(search_string)
        .arg(&config.base_filepath)
        .output()
        .expect("grep failed to start for some reason");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}


fn dispatch_display(entry: &String, config: &Config) -> Result<(), io::Error> {
    let now = Local::now();

    if entry.starts_with("show today") {
        let file_name = get_file_name(now.date());
        let mut path = config.base_filepath.clone();
        path.push_str(&file_name);

        print_file(path)

    } else if entry.starts_with("show yesterday") {
        let yesterday  = now.date() - Duration::days(1);
        let file_name = get_file_name(yesterday);
        let mut path = config.base_filepath.clone();
        path.push_str(&file_name);

        print_file(path)
    } else {

        // Cut of the "show" from the command
        let date = &entry[5..].trim();
        let mut file_name = String::new();
        file_name.push_str(&config.base_filepath);
        file_name.push_str(date);

        print_file(file_name)
    }
}

#[cfg(test)]
mod test {
    extern crate chrono;
    use self::chrono::*;
    use super::*;

    #[test]
    fn time_arithmetic() {
        let date = Local.ymd(2000, 1, 1);

        assert!(date + Duration::days(1) == Local.ymd(2000, 1, 2));
    }
}