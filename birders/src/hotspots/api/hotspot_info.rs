use crate::{errors::BirderError, hotspots::Hotspot, Birders};

pub struct HotspotInfoHandler<'birder> {
    birder: &'birder Birders,
    loc_id: String,
}

impl<'birder> HotspotInfoHandler<'birder> {
    pub fn new(birder: &'birder Birders, loc_id: &str) -> Self {
        Self {
            birder,
            loc_id: loc_id.to_string(),
        }
    }
}

impl<'birder> HotspotInfoHandler<'birder> {
    pub async fn get(&self) -> Result<Hotspot, BirderError> {
        let url = format!("/ref/hotspot/info/{}?fmt=json", self.loc_id);

        self.birder.get(&url).await
    }
}
