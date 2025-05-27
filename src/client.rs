use crate::error::HnError;
use crate::error::HttpError;
use crate::model::Id;
use futures::stream::FuturesUnordered;
use log;
use reqwest;
use reqwest::Client;
use reqwest::Request;
use reqwest::Response;
use serde_json;
use std::collections::VecDeque;
use std::error::Error;
// use crate::model::Thread;
// use crate::model::Listing;
use crate::model::firebase::Comment;
use crate::model::firebase::Item;
use crate::model::firebase::ItemsAndProfiles;
use crate::model::firebase::User;
use futures::stream::{self, StreamExt};
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinSet;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
pub struct HnClient {
    http_client: Client,
}

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[allow(clippy::new_without_default)]
impl HnClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    /// Send an HTTP request.
    async fn send(&self, req: Request) -> Result<Response, Box<dyn Error>> {
        let resp = self.http_client.execute(req).await?;
        let status = resp.status().as_u16();
        if status != 200 {
            let err = HttpError {
                url: resp.url().to_string(),
                code: status,
            };
            log::error!("Recieved non 200 status: {:?}", err);
            return Err(Box::new(HnError::HttpError(err)));
        }
        log::debug!("Recieved 200 status, response = {:?}", resp);

        Ok(resp)
    }

    /// Send an HTTP GET request.
    #[tracing::instrument(skip(self))]
    async fn get(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let req = self.http_client.get(url);
        let resp = self.send(req.build()?).await?;
        let status = resp.status();
        tracing::info!(status=?status);

        Ok(resp)
    }

    /// Retrieve an [Item] from the API.
    pub async fn item(&self, id: Id) -> Result<Item, Box<dyn Error>> {
        let url = format!("{base_url}/item/{id}.json", base_url = BASE_URL, id = id);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let item: Item = serde_json::from_str(&text)?;
        log::debug!("item = {:?}", item);

        Ok(item)
    }

    #[tracing::instrument(skip(self))]
    pub async fn thread(self: Arc<Self>, id: Id) -> Result<Item, Box<dyn Error>> {
        let item = self.item(id).await?;
        assert!(
            matches!(item, Item::Story(_)),
            "currently only support loading a thread from a story"
        );
        // let listing = Listing::from(item);
        // let comments = Vec::with_capacity(listing);
        // let mut thread = Thread { listings };
        // unimplemented!("have not implemented thread functionality");
        let _ = self._thread(id).await;

        Ok(item)
    }

    // compiles but doesn't work
    /// Retrieve a thread of comments
    pub async fn _thread(self: Arc<Self>, id: Id) -> HashMap<Id, Item> {
        let items = Arc::new(Mutex::new(HashMap::new()));
        let mut queue = VecDeque::from([id]);
        let mut in_flight = FuturesUnordered::new();

        loop {
            while let Some(id) = queue.pop_front() {
                tracing::debug!(id=?id, "initiating request");
                let client = self.clone();
                in_flight.push(async move { (id, client.item(id).await) });
            }

            match in_flight.next().await {
                Some((_id, Ok(item))) => {
                    let id = item.id();
                    tracing::debug!(item_id=?item.id(), "fetched item");
                    if let Some(kids) = item.kids() {
                        for kid in kids {
                            tracing::debug!(kid=?kid, "queueing new id");
                            queue.push_back(*kid);
                        }
                    }
                    items.lock().await.insert(id, item);
                }
                Some((id, Err(err))) => {
                    tracing::warn!(err=?err, id=?id, "fetch comment failed, requeue");
                    queue.push_back(id);
                }
                None => {
                    tracing::debug!("exhausted in_flight, breaking");
                    break;
                }
            }
        }

        let items = Arc::try_unwrap(items).unwrap().into_inner();

        items
    }

    pub async fn items(
        &self,
        ids: &[Id],
        // max_concurrent: Option<usize>
    ) -> Result<Vec<Item>, Box<dyn Error>> {
        let client = Arc::new(self);
        let limit = 10;

        // Convert the vector of IDs into a stream of futures
        let stream = stream::iter(ids)
            .map(move |id| {
                let client = client.clone();
                async move {
                    debug!("fetching item {:#?}", id);
                    let item = client.item(*id).await?;
                    debug!("finished item {:#?}", id);
                    Ok::<Item, Box<dyn Error>>(item)
                }
            })
            .buffered(limit)
            .collect::<Vec<Result<Item, Box<dyn Error>>>>()
            .await;
        stream
            .into_iter()
            .collect::<Result<Vec<Item>, Box<dyn Error>>>()
    }

    /// Retrieve the maximum [Item] from the API.
    pub async fn max_item(&self) -> Result<Id, Box<dyn Error>> {
        let url = format!("{base_url}/maxitem.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let id: Id = serde_json::from_str(&text)?;
        log::debug!("maxitem = {:?}", id);

        Ok(id)
    }

    pub async fn user(&self, username: String) -> Result<User, Box<dyn Error>> {
        let url = format!(
            "{base_url}/user/{id}.json",
            base_url = BASE_URL,
            id = username
        );

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let user: User = serde_json::from_str(&text)?;
        log::debug!("user = {:?}", user);

        Ok(user)
    }

    // TODO: newstorires and topstories are the exact same method with one substring changed.
    // Consider code consolidation, both with respect to these but all the API methods. For
    // example, would it be better for HackerNews Client API methods to create a request object,
    // and then have a single `request` method which executes a request?

    pub async fn new_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/newstories.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub async fn top_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/topstories.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub async fn updates(&self) -> Result<(Vec<Id>, Vec<String>), Box<dyn Error>> {
        let url = format!("{base_url}/updates.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let items_and_profiles: ItemsAndProfiles = serde_json::from_str(&text)?;
        let items = items_and_profiles.items;
        let profiles = items_and_profiles.profiles;
        let updates = (items, profiles);
        log::debug!("updates = {:?}", updates);

        Ok(updates)
    }

    pub async fn ask_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/askstories.json", base_url = BASE_URL,);
        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub async fn show_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/showstories.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }

    pub async fn job_stories(&self) -> Result<Vec<Id>, Box<dyn Error>> {
        let url = format!("{base_url}/jobstories.json", base_url = BASE_URL,);

        let resp = self.get(&url).await?;
        let text = resp.text().await?;
        log::debug!("text = {:?}", text);
        let ids: Vec<Id> = serde_json::from_str(&text)?;
        log::debug!("ids = {:?}", ids);

        Ok(ids)
    }
}

#[cfg(test)]
mod tests {

    use super::HnClient;
    use crate::model::firebase::Item;
    use crate::util::setup;
    use std::error::Error;

    #[tokio::test]
    async fn test_item() -> Result<(), Box<dyn Error>> {
        setup();

        let id_story = 27476206;
        let id_comment = 27509155;

        let client = HnClient::new();
        let story = client.item(id_story).await?;
        log::debug!("item = {:?}", story);
        assert!(story.is_story());

        let comment = client.item(id_comment).await?;
        log::debug!("item = {:?}", comment);
        assert!(comment.is_comment());

        Ok(())
    }

    #[tokio::test]
    async fn test_max_item() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let item = client.max_item().await?;
        log::debug!("maxitem = {:?}", item);

        Ok(())
    }

    #[tokio::test]
    async fn test_user() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let user = client.user("pg".to_string()).await?;
        log::debug!("user = {:?}", user);

        Ok(())
    }

    #[tokio::test]
    async fn test_new_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let ids = client.new_stories().await?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }

    #[tokio::test]
    async fn test_top_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let ids = client.top_stories().await?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }

    #[tokio::test]
    async fn test_updates() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let updates = client.updates().await?;
        log::debug!("updates = {:?}", updates);

        Ok(())
    }

    #[tokio::test]
    async fn test_ask_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let ids = client.ask_stories().await?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }

    #[tokio::test]
    async fn test_show_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let ids = client.show_stories().await?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }

    #[tokio::test]
    async fn test_job_stories() -> Result<(), Box<dyn Error>> {
        setup();

        let client = HnClient::new();
        let ids = client.job_stories().await?;
        log::debug!("ids = {:?}", ids);

        Ok(())
    }
}
