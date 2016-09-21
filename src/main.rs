mod parser;
mod logger;

fn main() {
    let arg_string = parser::get_args();

    match logger::log(arg_string) {
        Ok(()) => println!("Noted."),
        Err(_) => println!("Couldn't log entry. Better error handling to come."),
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }
}