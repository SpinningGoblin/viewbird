use crate::{errors::BirderError, observations::Observation, Birders};

use super::RecentInRegionParams;

pub struct RecentNotableInRegionHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
    params: Option<RecentInRegionParams>,
}

impl<'birder> RecentNotableInRegionHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        region_code: &str,
        params: Option<RecentInRegionParams>,
    ) -> Self {
        Self {
            birder,
            params,
            region_code: region_code.to_string(),
        }
    }
}

impl<'birder> RecentNotableInRegionHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!("/data/obs/{}/recent/notable", self.region_code);

        let full_url = match &self.params {
            Some(it) => format!("{}?{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
