use config::Config;
use logger::log;
use logger::get_file_name;
use std::io;

extern crate chrono;
use self::chrono::*;

pub fn dispatch(entry: String, config: &Config) -> Result<(), io::Error> {
    // if entry.starts_with("show") {
    //     dispatch_display(entry, config)
    // } else {
    //     log(entry, config)
    // }

    Ok(())
}

// fn dispatch_display(entry: String, config: &Config) -> Result<(), io::Error> {
//     let now = Local.now();
//     let mut file_name: String;

//     if entry.starts_with("show today") {
//         let file_name = logger::get_file_name(now);
//     }

//     if entry.starts_with("show yesterday") {
//         let file_name = "foo";
//     }
// }