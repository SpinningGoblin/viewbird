use crate::{errors::BirderError, observations::Observation, Birders, Location};

use super::NearSpeciesParams;

pub struct NearestOfSpeciesHandler<'birder> {
    birder: &'birder Birders,
    latitude: f64,
    longitude: f64,
    species_code: String,
    params: Option<NearSpeciesParams>,
}

impl<'birder> NearestOfSpeciesHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        species_code: &str,
        location: &Location,
        params: Option<NearSpeciesParams>,
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

impl<'birder> NearestOfSpeciesHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/nearest/geo/recent/{}?lat={}&lng={}",
            self.species_code, self.latitude, self.longitude
        );

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
