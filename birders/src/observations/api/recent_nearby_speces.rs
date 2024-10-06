use crate::{errors::BirderError, observations::Observation, Birders, Location};

use super::RecentNearbySpeciesParams;

pub struct RecentNearbySpeciesHandler<'birder> {
    birder: &'birder Birders,
    latitude: f64,
    longitude: f64,
    species_code: String,
    params: Option<RecentNearbySpeciesParams>,
}

impl<'birder> RecentNearbySpeciesHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        species_code: &str,
        location: &Location,
        params: Option<RecentNearbySpeciesParams>,
    ) -> Self {
        Self {
            birder,
            params,
            species_code: species_code.to_string(),
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }
}

impl<'birder> RecentNearbySpeciesHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/obs/geo/recent/{}?lat={}&lng={}",
            self.species_code, self.latitude, self.longitude
        );

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
