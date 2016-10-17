pub struct Config {
    pub base_filepath: String,
}

pub fn default_config() -> Config {
    Config {
        base_filepath: "/Users/az/logs/".to_owned(),
    }
}