use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct HNConfig {
    _user: Option<HNConfigUser>,
}

#[derive(Debug, Deserialize)]
struct HNConfigUser {
    _username: Option<String>,
}

impl HNConfig {
    pub fn from_file(path: &Path) -> Result<HNConfig, Box<dyn Error>> {
        let f = File::open(path)?;
        let rd = BufReader::new(f);
        let config = serde_json::from_reader(rd)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {

    use super::HNConfig;
    use std::error::Error;
    use std::path::PathBuf;

    // This test is marked as ignored, as the feature it supports is not
    // really implemented
    #[ignore]
    #[test]
    fn test_read_config_file() -> Result<(), Box<dyn Error>> {
        let home = std::env::var("HOME").expect("Failed to read `$HOME` environment variable");
        println!("$HOME = {:?}", home);
        let mut path = PathBuf::from(home);
        path.push(".hn.json");
        println!("path = {:?}", path);

        let config = HNConfig::from_file(&path)?;
        println!("config = {:#?}", config);

        Ok(())
    }
}
