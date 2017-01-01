use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::OpenOptions;

pub fn print_file(filename: String) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(&filename);

    match file {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            println!("");
            println!("{}", content);
            Ok(())
        }

        Err(e) => Err(e)
    }
}

pub fn archive(filename: String) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(&filename);

    match file {
        Ok(mut f) => {
            // Read the old file
            let mut content = String::new();
            f.read_to_string(&mut content)?;

            Ok(())
        },
        Err(e) => Err(e),
    }
}

pub fn append_to_file(filename: PathBuf, content: String) -> Result<usize, io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    file.write(content.as_bytes())
}