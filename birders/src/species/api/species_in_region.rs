use crate::{errors::BirderError, Birders};

pub struct SpeciesInRegionHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
}

impl<'birder> SpeciesInRegionHandler<'birder> {
    pub fn new(birder: &'birder Birders, region_code: &str) -> Self {
        Self {
            birder,
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> SpeciesInRegionHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<String>, BirderError> {
        self.birder
            .get(&format!("/product/spplist/{}", self.region_code))
            .await
    }
}
