use std::error::Error;
use log;
use serde_json;
use reqwest;
use reqwest::blocking::Client;
use reqwest::blocking::Request;
use reqwest::blocking::Response;
use crate::error::HnError;
use crate::error::HttpError;
use crate::model::Id;
use crate::model::firebase::User;
use crate::model::firebase::Item;
use crate::model::firebase::ItemsAndProfiles;
    
pub struct JsonClient {
    http_client: reqwest::blocking::Client,
}

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[allow(clippy::new_without_default)]
impl JsonClient {

    pub fn new() -> Self {
        Self {
            http_client: Client::new(), 
        }
    }
    
    fn send(&self, req: Request) -> Result<Response, Box<dyn Error>> {
        let resp = self.http_client.execute(req)?;
        let status = resp.status().as_u16();
        if status != 200 {
            let err = HttpError {
                url:  resp.url().to_string(),
                code: status,
            };
            log::error!("Recieved non 200 status: {:?}", err);
            return Err(Box::new(HnError::HttpError(err)));
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);

        Ok(resp)
    }
    
    fn get_url(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let req = self.http_client.get(url);
        let resp = self.send(req.build()?)?;

        Ok(resp)
    }

    pub fn item(&self, id: Id) -> Result <Item, Box<dyn Error>> {
        let url = format!("{base_url}/item/{id}.json",
            base_url=BASE_URL,
            id=id
        );

        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let item: Item = serde_json::from_str(&text)?;
        log::debug!("item = {:?}", item);

        Ok(item)
    }
    
    pub fn max_item(&self) -> Result <Id, Box<dyn Error>> {
        let url = format!("{base_url}/maxitem.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let id: Id = serde_json::from_str(&text)?;
        log::debug!("maxitem = {:?}", id);

        Ok(id)
    }

    pub fn user(&self, username: String) -> Result<User, Box<dyn Error>> {
        let url = format!("{base_url}/user/{id}.json",
            base_url=BASE_URL,
            id=username
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let user: User = serde_json::from_str(&text)?;
        log::debug!("user = {:?}", user);

        Ok(user)
    }

    // TODO: newstorires and topstories are the exact same method with one substring changed.
    // Consider code consolidation, both with respect to these but all the API methods. For
    // example, would it be better for HackerNews Client API methods to create a request object,
    // and then have a single `request` method which executes a request?

    pub fn new_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/newstories.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub fn top_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/topstories.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub fn updates(&self) -> Result<(Vec<Id>, Vec<String>), Box<dyn Error>> {
        let url = format!("{base_url}/updates.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let items_and_profiles: ItemsAndProfiles = serde_json::from_str(&text)?;
        let items = items_and_profiles.items;
        let profiles = items_and_profiles.profiles;
        let updates = (items, profiles);
        log::debug!("updates = {:?}", updates);

        Ok(updates)
    }

    pub fn ask_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/askstories.json",
            base_url=BASE_URL,
        );
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub fn show_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/showstories.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }
    
    pub fn job_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/jobstories.json",
            base_url=BASE_URL,
        );
        
        let resp = self.get_url(&url)?;
        let text = resp.text()?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

}

#[cfg(test)]
mod tests {

    use super::JsonClient;
    use std::error::Error;
    use crate::util::setup;

    #[test]
    fn test_item() -> Result<(), Box<dyn Error>> {
        setup();

        let id_story = 27476206;
        let id_comment = 27509155;

        let client = JsonClient::new();
        let story = client.item(id_story)?;
        log::debug!("item = {:?}", story);
        assert!(story.is_story());
        
        let comment = client.item(id_comment)?;
        log::debug!("item = {:?}", comment);
        assert!(comment.is_comment());

        Ok(())
    }

    #[test]
    fn test_max_item() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let item = client.max_item()?;
        log::debug!("maxitem = {:?}", item);

        Ok(())
    }
    
    #[test]
    fn test_user() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let user = client.user("pg".to_string())?;
        log::debug!("user = {:?}", user);

        Ok(())
    }
    
    #[test]
    fn test_new_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.new_stories()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
    
    #[test]
    fn test_top_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.top_stories()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
    
    #[test]
    fn test_updates() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let updates = client.updates()?;
        log::debug!("updates = {:?}", updates);

        Ok(())
    }
    
    #[test]
    fn test_ask_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.ask_stories()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
    
    #[test]
    fn test_show_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.show_stories()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }

    #[test]
    fn test_job_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.job_stories()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
}
