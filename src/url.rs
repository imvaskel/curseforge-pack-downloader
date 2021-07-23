use std::time::Duration;

use bytes::Bytes;
use reqwest::blocking;

use crate::models::{LatestFile, PackFileData, PackProject};

pub const BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2";
pub const GAME_ID: i32 = 432;
pub const SECTION_ID: i32 = 4471;

/// A blocking client that wraps the curseforge minecraft api.
pub struct Client {
    client: blocking::Client,
}

impl From<blocking::Client> for Client {
    /// Creates a new client from a reqwest blocking client.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to use.
    ///
    fn from(client: blocking::Client) -> Self {
        Self { client }
    }
}

impl Client {
    /// Creates a new client with a timeout of 60s.
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?;
        Ok(Self { client })
    }

    /// Creates a new client with the given timeout
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout.
    ///
    pub fn with_timeout(timeout: Duration) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: blocking::Client::builder().timeout(timeout).build()?,
        })
    }
}

impl Client {
    /// Searchs the api for the given term and returns a `Vec<PackProject>` of the results.
    ///
    /// # Arguments
    ///
    /// * `term` - The term to search with.
    /// * `max` - The max amount of results to display.
    ///
    pub fn search_packs(&self, term: &str, max: u32) -> Result<Vec<PackProject>, reqwest::Error> {
        let url = format!(
            "{}/addon/search?gameId={}&sectionId={}&pageSize={}&searchFilter={}",
            BASE_URL, GAME_ID, SECTION_ID, max, term
        );

        self.client.get(url).send()?.json()
    }

    /// Gets a packs data from the given id.
    ///
    /// # Arguments
    ///
    /// * `id` - The pack id.
    ///
    pub fn get_pack_data(&self, id: u32) -> Result<PackProject, reqwest::Error> {
        let url = format!("{}/addon/{}", BASE_URL, id);

        self.client.get(url).send()?.json()
    }

    /// Gets file data from the api.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The file's project id.
    /// * `file_id` - The file's id.
    ///
    pub fn get_file_data(
        &self,
        project_id: u32,
        file_id: u32,
    ) -> Result<PackFileData, reqwest::Error> {
        let url = format!("{}/addon/{}/file/{}/", BASE_URL, project_id, file_id);

        self.client.get(url).send()?.json()
    }

    /// Gets the given pack version's server pack url, if it exists.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project's id.
    /// * `file` - The file to get the server pack from.
    ///
    pub fn get_server_pack_url(
        &self,
        project_id: u32,
        file: &LatestFile,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}/addon/{}/file/{}/download-url",
            BASE_URL, project_id, file.server_pack_file_id
        );

        self.client.get(url).send()?.text()
    }

    /// Gets the server packs bytes.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to download from.
    ///
    pub fn download_server_pack(&self, url: &str) -> Result<Bytes, reqwest::Error> {
        self.client.get(url).send()?.bytes()
    }
}
