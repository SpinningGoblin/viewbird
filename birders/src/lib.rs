mod credentials;
pub mod errors;
pub mod models;

pub use credentials::Credentials;
use errors::BirderError;
use models::regions::{RegionType, SlimRegion};
use reqwest::{header, ClientBuilder};

pub struct Birders {
    client: reqwest::Client,
}

const BASE_URL: &str = "https://api.ebird.org/v2";

/// Constructors
impl Birders {
    pub fn new(credentials: Credentials) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut token_header = header::HeaderValue::from_str(&credentials.api_token).unwrap();
        token_header.set_sensitive(true);
        headers.insert("x-ebirdapitoken", token_header);
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }
}

impl Birders {
    pub async fn sub_region_list(
        &self,
        region_name: &str,
        sub_region_type: &RegionType,
    ) -> Result<Vec<SlimRegion>, BirderError> {
        let url = format!("{BASE_URL}/ref/region/list/{sub_region_type}/{region_name}");

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

/// Do the requests
impl Birders {}
