#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::error::Error;
    use std::io::Read;
    
    fn get_test_text() -> Result<String, Box<dyn Error>> {
          let mut f = File::open("../data/test.html")?;
          let mut buff = String::new();
          f.read_to_string(&mut buff)?;

          Ok(buff)
      }

    #[test]
    fn test_extract_comments() -> Result<(), Box<dyn Error>> {
        let text = get_test_text()?;
        let html = Html::parse_fragment(&text);
        let comments = extract_comments(&html)?;
        println!("comments = {:#?}", comments);

        Ok(())
    }

    #[test]
    fn test_comment_tree() -> Result<(), Box<dyn Error>> {
        let text = get_test_text()?;
        let html = Html::parse_document(&text);
        let comments = extract_comments(&html)?;
        let forest = create_comment_tree(comments);
        println!("forest = {:#?}", forest);

        Ok(())
    }
}
