#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use regex::Regex;
    use std::env;
    use std::error::Error;
    use std::fs;
    use std::path::PathBuf;

    // Note: There is an identical setup function in src/lib.rs; however, since integration tests
    // effectively use the crate as if it were an external dependancy, I don't think I can
    // import that test function.

    lazy_static! {
        static ref RE_LISTINGS_FILES: Regex = Regex::new("listings(.*).html")
            .expect("Failed to parse Regex instance: RE_LISTINGS_FILES");
        static ref RE_COMMENTS_FILES: Regex = Regex::new("comments(.*).html")
            .expect("Failed to parse Regex instance: RE_COMMENTS_FILES");
    }

    // fn data_dir() -> Result<PathBuf, Box<dyn Error>> {
    //     let mut data_dir = env::current_dir()?;
    //     data_dir.push("data");

    //     Ok(data_dir)
    // }

    // fn list_test_files(regex: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
    //     let filenames: Vec<String> = fs::read_dir(data_dir()?)?
    //         .filter_map(|path| path.ok())
    //         .filter_map(|dir_entry| dir_entry.file_name().into_string().ok())
    //         .filter(|path| regex.is_match(&path))
    //         .collect();

    //     Ok(filenames)
    // }

    // #[test]
    // fn test_listings() -> Result<(), Box<dyn Error>> {
    //     setup();

    //     for filename in  list_test_files(&RE_LISTINGS_FILES)? {
    //         log::info!("Starting integration test from listings file '{}'", filename);

    //         let mut path = data_dir()?;
    //         path.push(filename.clone());
    //         let mut f = File::open(path)?;
    //         let mut text = String::new();
    //         f.read_to_string(&mut text)?;
    //         let html = Html::parse_document(&text);
    //         let listings = ListingsParser::parse(&html)?;

    //         log::info!("Successfully parsed listings from file '{}'", filename);
    //         log::trace!("Listings parsed from '{}' = {:?}", filename, listings);
    //     }

    //     Ok(())
    // }

    // #[test]
    // fn test_comments() -> Result<(), Box<dyn Error>> {
    //     setup();

    //     for filename in  list_test_files(&RE_COMMENTS_FILES)? {
    //         log::info!("Starting integration test from comments file '{}'", filename);

    //         let mut path = data_dir()?;
    //         path.push(filename.clone());
    //         let mut f = File::open(path)?;
    //         let mut text = String::new();
    //         f.read_to_string(&mut text)?;
    //         let html = Html::parse_document(&text);
    //         let comments = CommentsParser::parse(&html)?;

    //         log::info!("Successfully parsed comments from file '{}'", filename);
    //         log::trace!("Comments parsed from '{}' = {:?}", filename, comments);
    //     }

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
