use crate::{errors::BirderError, observations::Observation, Birders, Location};

use super::RecentNearbyParams;

pub struct RecentNearbyHandler<'birder> {
    birder: &'birder Birders,
    latitude: f64,
    longitude: f64,
    params: Option<RecentNearbyParams>,
}

impl<'birder> RecentNearbyHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        location: &Location,
        params: Option<RecentNearbyParams>,
    ) -> Self {
        Self {
            birder,
            params,
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }
}

impl<'birder> RecentNearbyHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/obs/geo/recent?lat={}&lng={}",
            self.latitude, self.longitude
        );

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
