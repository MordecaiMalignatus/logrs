use std::io;
use std::io::Read;
use std::fs::OpenOptions;

pub fn print_file(filename: String) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(&filename);

    match file {
        Ok(mut f) => {
            let mut content = String::new();
            try!(f.read_to_string(&mut content));
            println!("{}", content);
            Ok(())
        }

        Err(e) => Err(e)
    }
}