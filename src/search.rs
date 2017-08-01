use config::Config;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::str;

use chrono::prelude::Local;
use ignore::WalkBuilder;
use grep::GrepBuilder;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ulid::Ulid;

// Borrowed libraries from ripgrep, namely ignore and grep for directory walking
// and line search. This is easier than including binaries.
// TODO: this function is extremely UGLY! Rewrite!
// TODO: add functionality for printing line numbers of matched files.
// TODO: try showing some lines above and below match to give context, this may come in handy when dealing with larger files.
pub fn custom_grep(pattern: &str, config: &Config) {
    let grep = GrepBuilder::new(pattern).build().unwrap();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for result in WalkBuilder::new(&config.base_filepath).hidden(false).build() {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                if entry.path().is_dir() {
                    continue;
                }

                // Buffer to read file into.
                let mut buf:Vec<u8> = Vec::new();
                let f = File::open(entry.path()).unwrap();
                let mut reader = BufReader::new(f);
                reader.read_to_end(&mut buf);

                let file_name = entry.path().file_name().unwrap().to_str().unwrap();

                let ulid = match Ulid::from_string(&file_name) {
                    Ok(ulid) => ulid,
                    Err(e) => {
                        eprintln!("Unable to parse ULID: {:?}", e);
                        return
                    }
                };
                let datetime = ulid.datetime().with_timezone(&Local);

                // Each match represents a line, with start and end byte position.
                for m in grep.iter(&buf) {
                    let mut last_written = 0;
                    if last_written == 0 {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                        write!(&mut stdout, "{} ", datetime);
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                        write!(&mut stdout, "({})", ulid.to_string());
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                        write!(&mut stdout, ":");
                    }

                    let buf2 = &buf[m.start()..m.end()];
                    for subm in grep.regex().find_iter(buf2) {
                        stdout.write(&buf2[last_written..subm.start()]);
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                        stdout.write(&buf2[subm.start()..subm.end()]);
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                        last_written = subm.end();
                    }
                    stdout.write(&buf2[last_written..]);
                }
            },
            Err(err) => println!("error: {}", err),
        }
    }
}