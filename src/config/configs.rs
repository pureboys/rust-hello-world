use std::fs;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mysql: Mysql,
}

#[derive(Deserialize, Debug)]
pub struct Mysql {
    pub host: String,
    pub port: String,
    pub user: String,
    pub passwd: String,
    pub db: String,
}

pub fn parse_toml(toml_path: String) -> Result<Config, anyhow::Error> {
    // 读取文件
    let toml_config = fs::read_to_string(toml_path)?;
    let config: Config = toml::from_str(&toml_config)?;
    Ok(config)
}

#[cfg(test)]
mod tests_config {
    use super::*;

    #[test]
    fn test_parse_toml() {
        parse_toml("config/config.toml".to_string()).unwrap();
    }
}