use std::error::Error;
use serde_json;
use reqwest;
use crate::model::Id;
use crate::model::Story;

pub struct JsonClient {
    http_client: reqwest::blocking::Client,
}

const URL_GET_BY_ID: &str = "https://hacker-news.firebaseio.com/v0/item";


impl JsonClient {

    pub fn new() -> Self {
        Self {
            http_client: reqwest::blocking::Client::new()
        }
    }


    pub fn get_by_id(&self, id: Id) -> Result <(), Box<dyn Error>> {
        let url = format!("{base_url}/{id}.json",
            base_url=URL_GET_BY_ID,
            id=id
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
        let text = resp.text()?;
        log::debug!("text = {:?}", text);

        let story: Story = serde_json::from_str(&text)?;
        log::debug!("story = {:?}", story);
        

        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::JsonClient;
    use std::error::Error;
    use crate::util::setup;

    #[test]
    fn test_get_by_id() -> Result<(), Box<dyn Error>> {
        setup();

        let id = 27476206;
        let client = JsonClient::new();
        client.get_by_id(id)?;

        Ok(())
    }

}
