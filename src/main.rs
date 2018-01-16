mod parser;
mod logger;
mod config;
mod dispatcher;
mod io;

#[macro_use]
extern crate serde_derive;


fn main() {
    let config = config::get_config();
    let arg_string = parser::get_args();

    match dispatcher::dispatch(arg_string, &config) {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }
}
