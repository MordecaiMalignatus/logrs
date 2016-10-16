mod parser;
mod logger;
mod config;
mod dispatcher;
mod io;

fn main() {
    let config = config::default_config();

    let arg_string = parser::get_args();

    match dispatcher::dispatch(arg_string, &config) {
        Ok(()) => {},
        Err(e) => println!("Couldn't log entry, Reason: {}", e),
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }
}