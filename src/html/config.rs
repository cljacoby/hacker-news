use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HNConfig {
    user: Option<HNConfigUser>
}

#[derive(Debug, Deserialize)]
struct HNConfigUser {
    username: Option<String>,
}

impl HNConfig {

    pub fn from_file(path: &Path) -> Result<HNConfig, Box<dyn Error>> {
        let f = File::open(path)?;
        let rd = BufReader::new(f);
        let config = serde_json::from_reader(rd)?;
    
        Ok(config)
    }

}

// "~/.config/hnews/hnews.json"
// fn read_config_file(path: &Path) -> Result<HNConfig, Box<dyn Error>> {
//     let f = File::open(path)?;
//     let rd = BufReader::new(f);
//     let config = serde_json::from_reader(rd)?;

//     Ok(config)
// }

#[cfg(test)]
mod tests {

    use std::path::Path;
    use std::path::PathBuf;
    use std::error::Error;
    use super::HNConfig;

    #[test]
    fn test_read_config_file() -> Result<(), Box<dyn Error>> {
        let home = std::env::var("HOME")
            .expect("Failed to read `$HOME` environment variable");
        println!("$HOME = {:?}", home);
        let mut path = PathBuf::from(home);
        path.push(".hnews.json");
        println!("path = {:?}", path);

        let config = HNConfig::from_file(&path)?;
        println!("config = {:#?}", config);

        Ok(())
    }

}