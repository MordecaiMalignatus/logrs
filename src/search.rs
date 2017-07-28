use config::Config;
use std::path::PathBuf;
use std::process::Command;

use chrono::prelude::Local;
use ulid::Ulid;

// TODO: figure out how to build/install/include ripgrep inside logrs.
// grep is nice, but it isn't available everywhere (looking at you Microsoft).
// ripgrep has the advantage of being able to compile it with Rust right into
// this program as a library (libripgrep) which is being worked on.
pub fn grep(pattern: &str, config: &Config) {
    let output = Command::new("rg")
        .arg("-i")
        .arg(pattern)
        .arg(&config.base_filepath)
        .output()
        .expect("ripgrep failed to start for some reason");

    // We iterate over lines because we need to match and replace file path with
    // more useful info - date and time + ULID.
    let out = String::from_utf8_lossy(&output.stdout);
    for line in out.lines() {
        // Split each line by ":", which is ripgrep terminator for filename:match.
        let parts: Vec<&str> = line.split(":").collect();
        let path = PathBuf::from(parts[0]);
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let ulid = match Ulid::from_string(&file_name) {
            Ok(ulid) => ulid,
            Err(e) => {
                eprintln!("Unable to parse ULID: {:?}", e);
                return
            }
        };
        let datetime = ulid.datetime().with_timezone(&Local);
        println!("{} ({}):{}", datetime, ulid.to_string(), parts[1]);
    }
}