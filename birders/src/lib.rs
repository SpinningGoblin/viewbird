pub mod api;
mod credentials;
pub mod errors;
pub mod models;

use api::regions::{RegionInfoHandler, SubRegionListHandler};
pub use credentials::Credentials;
use errors::BirderError;
use models::regions::RegionType;
use reqwest::{header, ClientBuilder};

pub struct Birders {
    client: reqwest::Client,
}

const BASE_URL: &str = "https://api.ebird.org/v2";
const API_TOKEN_HEADER: &str = "x-ebirdapitoken";
/// Constructors
impl Birders {
    pub fn new(credentials: Credentials) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut token_header = header::HeaderValue::from_str(&credentials.api_token)
            .expect("Invalid API token passed for header");
        token_header.set_sensitive(true);
        headers.insert(API_TOKEN_HEADER, token_header);
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }
}

impl Birders {
    pub fn sub_region_list(
        &self,
        region_code: &str,
        sub_region_type: RegionType,
    ) -> SubRegionListHandler {
        SubRegionListHandler::new(self, sub_region_type, region_code)
    }

    pub fn region_info(&self, region_code: &str) -> RegionInfoHandler {
        RegionInfoHandler::new(self, region_code)
    }
}

/// Do the requests
impl Birders {
    pub async fn get<T: for<'a> serde::Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<T, BirderError> {
        let url = format!("{BASE_URL}{path}");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(BirderError::EBirdRequestError)?;

        response
            .json()
            .await
            .map_err(|e| BirderError::UnknownError {
                message: e.to_string(),
            })
    }
}
