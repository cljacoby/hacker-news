#[cfg(test)]
mod tests {

    use std::fs;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;
    use std::error::Error;
    use std::sync::Once;
    use std::io::Read;
    use regex::Regex;
    use lazy_static::lazy_static;
    use scraper::Html;
    use hnews::parser::HtmlParse;
    use hnews::parser::ListingsParser;


    // Note: There is an identical setup function in src/lib.rs; however, since integration tests
    // effectively use the crate as if it were an external dependancy, I don't think I can
    // import that test function.

    static TEST_LOGGER: Once = Once::new(); 
    
    pub fn setup() {
        TEST_LOGGER.call_once(|| {
            // init_logger()
            env_logger::init();
        });
    }

    lazy_static! {
        static ref RE_LISTINGS_FILES: Regex = Regex::new("listings(.*).html")
            .expect("Failed to parse Regex instance: RE_LISTINGS_FILES");
    }

    #[test]
    fn test_listings() -> Result<(), Box<dyn Error>> {
        setup();

        let mut data_dir = env::current_dir()?;
        data_dir.push("data");
        let filenames: Vec<String> = fs::read_dir(data_dir.clone())?
            .filter_map(|path| path.ok())
            .filter_map(|dir_entry| dir_entry.file_name().into_string().ok())
            .filter(|path| RE_LISTINGS_FILES.is_match(&path))
            .collect();

        for filename in filenames {
            log::info!("Starting integration test from listings file '{}'", filename);
            let mut path = data_dir.clone();
            path.push(filename.clone());
            let mut f = File::open(path)?;
            let mut text = String::new();
            f.read_to_string(&mut text)?;
            let html = Html::parse_document(&text);
            let listings = ListingsParser::parse(&html)?; 
            log::info!("Successfully parsed listings from file '{}'", filename);
        }

        Ok(())
    }

    // log::debug!("data_dir = {:?}", data_dir);
    

    // #[test]
    // fn test_extract_comments() -> Result<(), Box<dyn Error>> {
    //     let text = get_test_text()?;
    //     let html = Html::parse_fragment(&text);
    //     let comments = extract_comments(&html)?;
    //     println!("comments = {:#?}", comments);
    //     Ok(())
    // }

    // #[test]
    // fn test_comment_tree() -> Result<(), Box<dyn Error>> {
    //     let text = get_test_text()?;
    //     let html = Html::parse_document(&text);
    //     let comments = extract_comments(&html)?;
    //     let forest = create_comment_tree(comments);
    //     println!("forest = {:#?}", forest);
    //     Ok(())
    // }
}
