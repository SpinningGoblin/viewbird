mod credentials;
pub mod errors;
pub mod hotspots;
mod location;
pub mod observations;
pub mod regions;

pub use credentials::Credentials;
use errors::BirderError;
use hotspots::api::{
    HotspotInfoHandler, HotspotsInRegionHandler, NearbyHotspotsHandler, NearbyParams,
};
pub use location::Location;
use observations::api::{
    HistoricOnDateHandler, HistoricOnDateParams, NearSpeciesParams, NearestOfSpeciesHandler,
    RecentInRegionForSpeciesHandler, RecentInRegionHandler, RecentInRegionParams,
    RecentNearbyHandler, RecentNearbyNotableHandler, RecentNearbyNotableParams, RecentNearbyParams,
    RecentNearbySpeciesHandler, RecentNotableInRegionHandler,
};
use regions::{
    api::{AdjacentRegionHandler, RegionInfoHandler, SubRegionListHandler},
    RegionType,
};
use reqwest::{header, ClientBuilder};
use time::Date;

pub struct Birders {
    client: reqwest::Client,
    debug_printing: bool,
}

const BASE_URL: &str = "https://api.ebird.org/v2";
const API_TOKEN_HEADER: &str = "x-ebirdapitoken";

/// Constructors
impl Birders {
    fn create_client(credentials: Credentials) -> reqwest::Client {
        let mut headers = header::HeaderMap::new();
        let mut token_header = header::HeaderValue::from_str(&credentials.api_token)
            .expect("Invalid API token passed for header");
        token_header.set_sensitive(true);
        headers.insert(API_TOKEN_HEADER, token_header);
        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    }

    pub fn new(credentials: Credentials) -> Self {
        let client = Self::create_client(credentials);
        Self {
            client,
            debug_printing: false,
        }
    }

    pub fn new_with_debug_printing(credentials: Credentials) -> Self {
        let client = Self::create_client(credentials);
        Self {
            client,
            debug_printing: true,
        }
    }
}

impl Birders {
    pub fn adjacent_regions(&self, region_code: &str) -> AdjacentRegionHandler {
        AdjacentRegionHandler::new(self, region_code)
    }

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

    pub fn hotspots_in_region(
        &self,
        region_code: &str,
        back: Option<u8>,
    ) -> HotspotsInRegionHandler {
        HotspotsInRegionHandler::new(self, region_code, back)
    }

    pub fn hotspot_info(&self, loc_id: &str) -> HotspotInfoHandler {
        HotspotInfoHandler::new(self, loc_id)
    }

    pub fn nearby_hotspots(
        &self,
        location: &Location,
        params: Option<NearbyParams>,
    ) -> NearbyHotspotsHandler {
        NearbyHotspotsHandler::new(self, location, params)
    }

    pub fn recent_observations_in_region(
        &self,
        region_code: &str,
        params: Option<RecentInRegionParams>,
    ) -> RecentInRegionHandler {
        RecentInRegionHandler::new(self, region_code, params)
    }

    pub fn recent_notable_observations_in_region(
        &self,
        region_code: &str,
        params: Option<RecentInRegionParams>,
    ) -> RecentNotableInRegionHandler {
        RecentNotableInRegionHandler::new(self, region_code, params)
    }

    pub fn recent_observations_for_species_in_region(
        &self,
        region_code: &str,
        species_code: &str,
        params: Option<RecentInRegionParams>,
    ) -> RecentInRegionForSpeciesHandler {
        RecentInRegionForSpeciesHandler::new(self, region_code, species_code, params)
    }

    pub fn recent_nearby_observations(
        &self,
        location: &Location,
        params: Option<RecentNearbyParams>,
    ) -> RecentNearbyHandler {
        RecentNearbyHandler::new(self, location, params)
    }

    pub fn recent_nearby_species_observations(
        &self,
        species_code: &str,
        location: &Location,
        params: Option<NearSpeciesParams>,
    ) -> RecentNearbySpeciesHandler {
        RecentNearbySpeciesHandler::new(self, species_code, location, params)
    }

    pub fn nearest_species_observations(
        &self,
        species_code: &str,
        location: &Location,
        params: Option<NearSpeciesParams>,
    ) -> NearestOfSpeciesHandler {
        NearestOfSpeciesHandler::new(self, species_code, location, params)
    }

    pub fn recent_nearby_notable_observations(
        &self,
        location: &Location,
        params: Option<RecentNearbyNotableParams>,
    ) -> RecentNearbyNotableHandler {
        RecentNearbyNotableHandler::new(self, location, params)
    }

    pub fn historic_observations_on_date(
        &self,
        region_code: &str,
        date: &Date,
        params: Option<HistoricOnDateParams>,
    ) -> HistoricOnDateHandler {
        HistoricOnDateHandler::new(self, region_code, date, params)
    }
}

/// Do the requests
impl Birders {
    pub async fn get<T: for<'a> serde::Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<T, BirderError> {
        let url = format!("{BASE_URL}{path}");

        if self.debug_printing {
            println!("{url}");
        }

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(BirderError::EBirdRequestError)?;

        let status_code = response.status();
        let text = response.text().await.unwrap();
        if !status_code.is_success() {
            return Err(BirderError::EBirdErrorResponse {
                body: text,
                status: status_code,
            });
        }

        if self.debug_printing {
            println!("{text}");
        }
        serde_json::from_str(&text).map_err(|e| BirderError::UnknownError {
            message: e.to_string(),
        })
    }
}
