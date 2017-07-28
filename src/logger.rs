use std::fs::{File, create_dir};
use std::io;
use std::io::Write;
use std::path::Path;

use config::Config;

use chrono::prelude::Local;
use ulid::Ulid;

pub fn log(entry: &str, config: &Config) -> Result<(), io::Error> {
    let ulid = Ulid::new();
    // We use ULID for generating datetime of file creation, it is also used as
    // name for the created log file. We must convert ULID datetime into Local TZ
    // otherwise there could be problems with having some files in incorrect directory
    // (cases when recording at 23:59 Local time).
    // ULID stamp will still remain in UTC, but we are aware of that, and we must
    // convert it into Local TZ in all places where we display datetime from ULID.
    // We could create ULID timestamps with Local TZ, but there could be problems
    // when traveling (TZ would change and mess up your logs), which we want to avoid.
    let date = ulid.datetime().with_timezone(&Local).date();
    let date_str = date.format("%Y-%m-%d").to_string();
    let today_dir = Path::new(&config.base_filepath).join(date_str);

    if !today_dir.exists() {
        match create_dir(&today_dir) {
            Ok(_) => {},
            Err(err) => return Err(err)
        }
    }

    let filepath = today_dir.join(ulid.to_string());

    let mut file = match File::create(&filepath) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    match file.write_all(entry.as_bytes()) {
        Ok(_) => {
            println!("Noted. {}", filepath.display());
            Ok(())
        }
        Err(err) => Err(err)
    }
}