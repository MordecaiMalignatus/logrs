use std::env;

pub fn get_args() -> String {
    let mut args = env::args();
    // skip file location, not needed.
    args.next();
    let mut res = String::new();

    for arg in args {
        res.push_str(&arg);
        res.push_str(" ");
    }

    res
}