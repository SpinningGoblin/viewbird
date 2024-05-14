use crate::{errors::BirderError, hotspots::HotspotGeo, Birders};

pub struct HotspotsInRegionHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
    back: Option<u8>,
}

impl<'birder> HotspotsInRegionHandler<'birder> {
    pub fn new(birder: &'birder Birders, region_code: &str, back: Option<u8>) -> Self {
        Self {
            birder,
            region_code: region_code.to_string(),
            back,
        }
    }
}

impl<'birder> HotspotsInRegionHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<HotspotGeo>, BirderError> {
        let back_param = self.back.filter(|val| *val <= 30);
        let url = format!("/ref/hotspot/{}?fmt=json", self.region_code);

        let full_url = match back_param {
            Some(it) => format!("{}&back={}", url, it),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
