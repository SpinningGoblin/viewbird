use crate::{errors::BirderError, regions::SlimRegion, Birders};

pub struct AdjacentRegionHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
}

impl<'birder> AdjacentRegionHandler<'birder> {
    pub fn new(birder: &'birder Birders, region_code: &str) -> Self {
        Self {
            birder,
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> AdjacentRegionHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<SlimRegion>, BirderError> {
        self.birder
            .get(&format!("/ref/adjacent/{}", self.region_code))
            .await
    }
}
