use crate::{errors::BirderError, observations::Observation, Birders};

use super::RecentInRegionParams;

pub struct RecentInRegionForSpeciesHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
    species_code: String,
    params: Option<RecentInRegionParams>,
}

impl<'birder> RecentInRegionForSpeciesHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        region_code: &str,
        species_code: &str,
        params: Option<RecentInRegionParams>,
    ) -> Self {
        Self {
            birder,
            params,
            species_code: species_code.to_string(),
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> RecentInRegionForSpeciesHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/obs/{}/recent/{}",
            self.region_code, self.species_code
        );

        let full_url = match &self.params {
            Some(it) => format!("{}?{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
