use crate::{errors::BirderError, hotspots::HotspotGeo, Birders, Location};

/// Optional parameters for nearby hotspots.
pub struct NearbyParams {
    /// How many days back to look up data for. Max is 30.
    /// Will default to None if not provided.
    pub back: Option<u8>,
    /// Search radius from the location, in kilometers.
    /// Must be in range 0-500.
    /// Will default to whatever API default is (25 at time of writing).
    pub dist: Option<u16>,
}

impl NearbyParams {
    fn to_url(&self) -> Vec<String> {
        let mut url_params = Vec::new();

        if let Some(back) = self.back {
            url_params.push(format!("back={back}"));
        }

        if let Some(dist) = self.dist {
            url_params.push(format!("dist={dist}"));
        }

        url_params
    }
}

pub struct NearbyHotspotsHandler<'birder> {
    birder: &'birder Birders,
    location: Location,
    params: Option<NearbyParams>,
}

impl<'birder> NearbyHotspotsHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        location: &Location,
        params: Option<NearbyParams>,
    ) -> Self {
        Self {
            birder,
            location: *location,
            params,
        }
    }
}

impl<'birder> NearbyHotspotsHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<HotspotGeo>, BirderError> {
        let url = format!(
            "/ref/hotspot/geo?fmt=json&lat={}&lng={}",
            self.location.latitude, self.location.longitude
        );

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
