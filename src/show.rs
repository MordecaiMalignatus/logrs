use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

use config::Config;

use chrono::prelude::{Local};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use time::Duration;
use ulid::Ulid;

pub fn show_today(config: &Config) {
    let today = Local::now().date().format("%Y-%m-%d").to_string();
    let dir = Path::new(&config.base_filepath).join(today);

    if !dir.exists() {
        println!("No records for today.");
        return
    }

    show_dir(dir);
}

pub fn show_yesterday(config: &Config) {
    let day = Duration::days(1);
    let yesterday = Local::now().date() - day;
    let dir = Path::new(&config.base_filepath).join(yesterday.format("%Y-%m-%d").to_string());

    if !dir.exists() {
        println!("No records for yesterday.");
        return
    }

    show_dir(dir);
}

pub fn show_date(date: &str, config: &Config) {
    let dir = Path::new(&config.base_filepath).join(date);

    if !dir.exists() {
        println!("No records for {}.", date);
        return
    }

    show_dir(dir);
}

pub fn show_id(id: &str, config: &Config) {
    // Parse ULID and extract date from it, it is done second time
    // in show_file function, but we need ULID parsed for the date
    // which is also the directory where this file is.
    let ulid = match Ulid::from_string(id) {
        Ok(ulid) => ulid,
        Err(e) => {
            eprintln!("Unable to parse ULID: {:?}", e);
            return
        }
    };
    let date = ulid.datetime().with_timezone(&Local).date();
    let path = Path::new(&config.base_filepath).join(date.format("%Y-%m-%d").to_string()).join(id);

    if !path.exists() {
        eprintln!("Record {} does not exists!", ulid.to_string());
        return
    }

    show_file(path);
}

fn show_dir(dir: PathBuf) {
    match read_dir(dir){
        Ok(dir_iterator) => {
            for entry in dir_iterator {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Unable to read directory entry: {}", e);
                        continue;
                    }
                };
                let path = entry.path();
                if path.is_dir(){
                    continue;
                }
                
                show_file(path);
            }
        },
        Err(e) => eprintln!("Unable to read directory: {}", e),
    }
}

// TODO: maybe limit the size of file displayed to N lines when showing with "show today"
// and show entire record when ID is given - try this how it works.
fn show_file(path: PathBuf) {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open file: {}, error: {}", &path.display(), e);
            return
        }
    };
    // There may be a more efficient way to read files and print them to stdin, without
    // using a buffer.
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Unable to read file: {}, error: {}", &path.display(), e);
            return
        }
    };
    // We probably can unwrap safely here, all files should have ULID names, which are 100% Unicode compatible.
    let file_ulid = path.file_name().unwrap().to_str().unwrap();
    // Convert filename from ULID into datetime and convert to Local TZ.
    let ulid = match Ulid::from_string(&file_ulid) {
        Ok(ulid) => ulid,
        Err(e) => {
            eprintln!("Unable to parse ULID: {:?}", e);
            return
        }
    };
    let datetime = ulid.datetime().with_timezone(&Local);
    // Colorizing the output. This is extremely ugly, anyone got ideas how
    // to make this nicer?
    let mut stdout = StandardStream::stdout(ColorChoice::Always); 
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    write!(&mut stdout, "{} ", datetime);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    write!(&mut stdout, "({})\n", ulid.to_string());
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    println!("{}", contents);
    println!("");
}