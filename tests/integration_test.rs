use hnews::error::HNError;
use hnews::models::Item;
use serde_json;
use serde_json::Value;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/*
 * Using a full dump of HackerNews data from October 2006 to
 * May 2018 for testing. Files available from:
 *
 * https://files.pushshift.io/hackernews/
 * */

const TEST_DATA_DIR: &str = "./data";

#[test]
fn integration_test() -> Result<(), Box<dyn Error>> {
    let mut err_count = 0;
    let test_data_dir = Path::new(TEST_DATA_DIR);

    for entry in fs::read_dir(test_data_dir)? {
        let entry = entry?;
        let path = entry.path();
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        // This is not performant. I'm parsing twice, and cloning a value in the
        // middle. However, parsing the JSON first to Values, and then in to
        // the custom hnews::Item type allows each parse to be handled independently.
        // Otherwise, if you use serde_json to parse directly to a Vec<hnews::Item>,
        // the entire parse will fail at the first error.

        let values: Vec<Value> = serde_json::from_reader(reader)?;
        for v in values.iter() {
            match serde_json::from_value::<Item>(v.clone()) {
                Ok(_) => continue,
                Err(err) => {
                    err_count += 1;
                    println!("error = {:?}. value = {:?}", err, v);
                }
            }
        }
    }

    match err_count {
        0 => Ok(()),
        _ => {
            let err = HNError::new(
                format!(
                    "Integrationt test failed with {} messages failed to parse",
                    err_count
                ),
                None,
            );

            Err(Box::new(err))
        }
    }
}
