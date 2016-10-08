use std::io;
use std::io::Read;
use std::fs::OpenOptions;

pub fn print_file(filename: String) -> Result<(), io::Error> {
    let file = OpenOptions::new().read(true).open(&filename).unwrap();
    let mut content = String::new();

    match file {
        Ok(mut f) => {
            try!(f.read_to_string(&mut content));
            println!("{:?}", content);
            Ok(())
        }

        Err(e) => {
            println!("Error reading file at {}, error: \n {}", filename, e);
            Err(e)
        }
    }
}