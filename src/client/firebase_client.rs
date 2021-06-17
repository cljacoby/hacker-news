use std::error::Error;
use serde_json;
use reqwest;
use crate::model::Id;
use crate::model::firebase::User;
use crate::model::firebase::Item;
use crate::model::firebase::ItemsAndProfiles;
    

// [DONE] Get User By ID
// [DONE] Max Item ID
// [DONE] New and Top Stories
// Ask HN Stories
// Show HN Stories
// Job HN Stories
// [DONE] Changed Items and Profiles


pub struct JsonClient {
    http_client: reqwest::blocking::Client,
}

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

impl JsonClient {

    pub fn new() -> Self {
        Self {
            http_client: reqwest::blocking::Client::new()
        }
    }

    pub fn get_item(&self, id: Id) -> Result <Item, Box<dyn Error>> {
        let url = format!("{base_url}/item/{id}.json",
            base_url=BASE_URL,
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

        let item: Item = serde_json::from_str(&text)?;
        log::debug!("item = {:?}", item);
        

        Ok(item)
    }
    
    pub fn max_item_id(&self) -> Result <Id, Box<dyn Error>> {
        let url = format!("{base_url}/maxitem.json",
            base_url=BASE_URL,
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
        let text = resp.text()?;
        log::debug!("text = {:?}", text);

        let id: Id = serde_json::from_str(&text)?;
        log::debug!("maxitem = {:?}", id);

        Ok(id)
    }

    pub fn get_user(&self, username: String) -> Result<User, Box<dyn Error>> {
        let url = format!("{base_url}/user/{id}.json",
            base_url=BASE_URL,
            id=username
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
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

    pub fn new_story_ids(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/newstories.json",
            base_url=BASE_URL,
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
        let text = resp.text()?;
        log::debug!("text = {:?}", text);

        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub fn top_story_ids(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/topstories.json",
            base_url=BASE_URL,
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
        let text = resp.text()?;
        log::debug!("text = {:?}", text);

        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub fn get_updates(&self) -> Result<(Vec<Id>, Vec<String>), Box<dyn Error>> {
        let url = format!("{base_url}/updates.json",
            base_url=BASE_URL,
        );
        
        let req = self.http_client.get(&url);
        let resp = req.send()?;
        if resp.status().as_u16() != 200 {
            log::error!("Recieved non 200 status, response = {:?}", resp);
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);
        let text = resp.text()?;
        log::debug!("text = {:?}", text);

        let items_and_profiles: ItemsAndProfiles = serde_json::from_str(&text)?;
        let items = items_and_profiles.items;
        let profiles = items_and_profiles.profiles;
        let updates = (items, profiles);
        log::debug!("updates = {:?}", updates);

        Ok(updates)
    }

}

#[cfg(test)]
mod tests {

    use super::JsonClient;
    use std::error::Error;
    use crate::util::setup;

    #[test]
    fn test_get_item() -> Result<(), Box<dyn Error>> {
        setup();

        let id_story = 27476206;
        let id_comment = 27509155;

        let client = JsonClient::new();
        let story = client.get_item(id_story)?;
        log::debug!("item = {:?}", story);
        assert!(story.is_story());
        
        let comment = client.get_item(id_comment)?;
        log::debug!("item = {:?}", comment);
        assert!(comment.is_comment());

        Ok(())
    }

    #[test]
    fn test_max_item_id() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let item = client.max_item_id()?;
        log::debug!("maxitem = {:?}", item);

        Ok(())
    }
    
    #[test]
    fn test_get_user() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let user = client.get_user("pg".to_string())?;
        log::debug!("user = {:?}", user);

        Ok(())
    }
    
    #[test]
    fn test_get_top_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let ids = client.top_story_ids()?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
    
    #[test]
    fn test_get_updates() -> Result<(), Box<dyn Error>> {
        setup();

        let client = JsonClient::new();
        let updates = client.get_updates()?;
        log::debug!("updates = {:?}", updates);

        Ok(())
    }

}
