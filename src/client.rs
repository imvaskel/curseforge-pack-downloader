use crate::models::{PackFileData, PackProject};
use std::time::Duration;

pub const BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2";
pub const GAME_ID: i32 = 432;
pub const SECTION_ID: i32 = 4471;

pub struct Client {
    pub http: reqwest::Client,
}

impl From<reqwest::Client> for Client {
    fn from(client: reqwest::Client) -> Self {
        Self { http: client }
    }
}

impl Client {
    /// Creates a new client with a timeout of 60s.
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self::with_timeout(Duration::from_secs(60))?)
    }

    /// Creates a new client with the given timeout
    ///
    /// # Arguments
    ///
    /// * `duration` - the timeout duration.
    pub fn with_timeout(duration: Duration) -> Result<Self, reqwest::Error> {
        Ok(Self {
            http: reqwest::ClientBuilder::new().timeout(duration).build()?,
        })
    }
}

impl Client {
    ///
    /// Searchs the api for a given term with a page size of x.
    ///
    /// # Arguments
    ///
    /// * `term` - the term to search with.
    /// * `page_size` - the page size to return.
    pub async fn search_packs(
        &self,
        term: &str,
        page_size: u64,
    ) -> Result<Vec<PackProject>, reqwest::Error> {
        let url = format!(
            "{}/addon/search?gameId={}&sectionId={}&pageSize={}&searchFilter={}",
            BASE_URL, GAME_ID, SECTION_ID, page_size, term
        );

        Ok(self.http.get(url).send().await?.json().await?)
    }

    /// Gets a packs data from the given id
    /// # Arguments
    ///
    /// * `id` - the pack id.
    pub async fn get_pack_data(&self, id: u64) -> Result<PackProject, reqwest::Error> {
        let url = format!("{}/addon/{}", BASE_URL, id);

        Ok(self.http.get(url).send().await?.json().await?)
    }

    /// Gets a pack file data from the given project id and file id,
    ///
    /// # Arguments
    ///
    /// * `project_id` - the project id of the file.
    /// * `file_id` - the file id.
    pub async fn get_file_data(
        &self,
        project_id: u64,
        file_id: u64,
    ) -> Result<PackFileData, reqwest::Error> {
        let url = format!("{}/addon/{}/file/{}", BASE_URL, project_id, file_id);

        Ok(self.http.get(url).send().await?.json().await?)
    }

    /// Gets a download url from the given project id and file id. This works for both server packs and client packs.
    ///
    /// # Arguments
    ///
    /// * `project_id` - the project id.
    /// * `file_id` - the file id.
    pub async fn get_download_url(
        &self,
        project_id: u64,
        file_id: u64,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}/addon/{}/file/{}/download-url",
            BASE_URL, project_id, file_id
        );

        Ok(self.http.get(url).send().await?.text().await?)
    }
}
