use std::{collections::HashMap, fmt};

use reqwest::StatusCode;
use serde::{de, Serialize};
use url::Url;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GetError(#[from] reqwest::Error),
    #[error("Faile get emotes {0}")]
    GETAPIErorr(APIError),
}

#[derive(Debug)]
pub struct APIError {
    pub status: String,
    pub message: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed get emotes")
    }
}

#[derive(Debug, PartialEq)]
pub struct APIBase {
    access_token: String,
    client_id: String,
    pub base_url: Url,
}

impl APIBase {
    pub fn new<T: Into<String>>(access_token: T, client_id: T, base_url: T) -> APIBase {
        APIBase {
            access_token: access_token.into(),
            client_id: client_id.into(),
            base_url: Url::parse(&base_url.into()).unwrap(),
        }
    }

    pub async fn api_request_text<T: Into<String>>(&self, url: T) -> Result<String> {
        let req = reqwest::Client::new()
            .get(url.into())
            .header("Authorization", self.header_auth())
            .header("Client-Id", &self.client_id)
            .send()
            .await?;

        let status = req.status();

        match status {
            StatusCode::OK => Ok(req.text().await?),
            _ => Err(Error::GETAPIErorr(APIError {
                status: status.to_string(),
                message: req.text().await?,
            })),
        }
    }

    pub async fn api_request_json<T: Into<String>, S: de::DeserializeOwned>(
        &self,
        url: T,
    ) -> Result<S> {
        let req = reqwest::Client::new()
            .get(url.into())
            .header("Authorization", self.header_auth())
            .header("Client-Id", &self.client_id)
            .send()
            .await?;

        let status = req.status();

        match status {
            StatusCode::OK => Ok(req.json().await?),
            _ => Err(Error::GETAPIErorr(APIError {
                status: status.to_string(),
                message: req.text().await?,
            })),
        }
    }

    pub async fn api_post<T: Into<String>, I: Serialize>(
        &self,
        url: T,
        body: &I,
    ) -> Result<String> {
        let req = reqwest::Client::new()
            .post(url.into())
            .header("Authorization", self.header_auth())
            .header("Client-Id", &self.client_id)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let status = req.status();

        match status {
            StatusCode::OK => Ok(req.text().await?),
            _ => Err(Error::GETAPIErorr(APIError {
                status: status.to_string(),
                message: req.text().await?,
            })),
        }
    }

    pub async fn api_post_json<T: Into<String>, I: Serialize, S: de::DeserializeOwned>(
        &self,
        url: T,
        body: &I,
    ) -> Result<S> {
        let req = reqwest::Client::new()
            .post(url.into())
            .header("Authorization", self.header_auth())
            .header("Client-Id", &self.client_id)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let status = req.status();

        match status {
            StatusCode::OK => Ok(req.json().await?),
            _ => Err(Error::GETAPIErorr(APIError {
                status: status.to_string(),
                message: req.text().await?,
            })),
        }
    }

    pub async fn api_delete<T: Into<String>>(&self, url: T) -> Result<String> {
        let req = reqwest::Client::new()
            .delete(url.into())
            .header("Authorization", self.header_auth())
            .header("Client-Id", &self.client_id)
            .send()
            .await?;

        let status = req.status();

        match status {
            StatusCode::OK => Ok(req.text().await?),
            _ => Err(Error::GETAPIErorr(APIError {
                status: status.to_string(),
                message: req.text().await?,
            })),
        }
    }

    fn header_auth(&self) -> String {
        format!("Bearer {}", &self.access_token)
    }

    pub fn url(&self) -> String {
        format!("{}", self.base_url)
    }

    pub fn url_endpoint<T: Into<String>>(&self, endpoint: T) -> String {
        format!("{}/{}", self.base_url, endpoint.into())
    }

    pub fn url_qurey<T: Into<String>>(&self, key: T, value: T) -> String {
        format!(
            "{}?{}",
            self.base_url,
            self.set_query(vec![(key.into(), value.into())])
        )
    }
    pub fn url_qureys<T: Into<String>>(&self, query: Vec<(T, T)>) -> String {
        format!("{}?{}", self.base_url, self.set_query(query))
    }

    pub fn url_endpoint_qurey<T: Into<String>>(&self, endpoint: T, key: T, value: T) -> String {
        format!(
            "{}/{}?{}",
            self.base_url,
            endpoint.into(),
            self.set_query(vec![(key.into(), value.into())])
        )
    }

    pub fn url_endpoint_qureys<T: Into<String>>(
        &self,
        endpoint: T,
        query: Vec<(&str, &str)>,
    ) -> String {
        format!(
            "{}/{}?{}",
            self.base_url,
            endpoint.into(),
            self.set_query(query)
        )
    }

    fn set_query<T: Into<String>>(&self, query: Vec<(T, T)>) -> String {
        query
            .into_iter()
            .map(|(key, value)| format!("{}={}", key.into(), value.into()))
            .collect::<Vec<String>>()
            .join("&")
    }
}

#[derive(Debug, Serialize)]
pub struct APIPostBoby {
    pub data: HashMap<String, String>,
}

impl APIPostBoby {
    pub fn new(data: HashMap<String, String>) -> APIPostBoby {
        APIPostBoby { data }
    }
}
