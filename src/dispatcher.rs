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
        // ref c if c.starts_with("archive") => dispatch_archiving(config),
        _ => logger::log(&entry, config),
    }
}

// fn dispatch_archiving(config: &Config) -> Result<(), io::Error> {
//     println!("Not implemented just yet. :)");
//     Ok(())
// }


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
    let mut path = config.base_filepath.clone();

    if entry.starts_with("show today") {
        let file_name = get_file_name(now.date());
        path.push(&file_name);
        print_file(path)

    } else if entry.starts_with("show yesterday") {
        let yesterday = now.date() - Duration::days(1);
        let file_name = get_file_name(yesterday);
        path.push(&file_name);

        print_file(path)
    } else {
        // Cut of the "show" from the command
        let date = &entry[5..].trim();
        path.push(date);

        print_file(path)
    }
}
