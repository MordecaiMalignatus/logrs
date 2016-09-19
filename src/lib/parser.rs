use std::env;

pub fn get_args() -> String {
    // skip file location, not needed.
    let args = env::args().next();
    let mut res = String::new();

    for arg in args {
        res.push_str(" ");
        res.push_str(&arg);
    }

    res
}