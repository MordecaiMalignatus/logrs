pub struct Config {
    pub base_filepath: String,
    pub line_width: i32,
}

pub fn default_config() -> Config {
    Config {
        base_filepath: "/Users/az/logs/".to_owned(),
        line_width: 80,
    }
}