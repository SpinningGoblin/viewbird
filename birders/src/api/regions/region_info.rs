use crate::{errors::BirderError, models::regions::Region, Birders};

pub struct RegionInfoHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
}

impl<'birder> RegionInfoHandler<'birder> {
    pub fn new(birder: &'birder Birders, region_code: &str) -> Self {
        Self {
            birder,
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> RegionInfoHandler<'birder> {
    pub async fn get(&self) -> Result<Region, BirderError> {
        self.birder
            .get(&format!("/ref/region/info/{}", self.region_code))
            .await
    }
}
