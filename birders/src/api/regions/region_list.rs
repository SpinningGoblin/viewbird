use crate::{
    errors::BirderError,
    models::regions::{RegionType, SlimRegion},
    Birders,
};

pub struct SubRegionListHandler<'birder> {
    birder: &'birder Birders,
    sub_region_type: RegionType,
    region_code: String,
}

impl<'birder> SubRegionListHandler<'birder> {
    pub fn new(birder: &'birder Birders, sub_region_type: RegionType, region_code: &str) -> Self {
        Self {
            birder,
            sub_region_type,
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> SubRegionListHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<SlimRegion>, BirderError> {
        self.birder
            .get(&format!(
                "/ref/region/list/{}/{}",
                self.sub_region_type, self.region_code
            ))
            .await
    }
}
