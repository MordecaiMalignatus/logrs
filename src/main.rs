mod lib;

fn main() {
    println!("{}", lib::parser::get_args());
}

#[cfg(test)]
mod test{
    #[test]
    fn test_sanity() {
        assert!(1 + 1 == 2);
    }
}