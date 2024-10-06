use crate::{errors::BirderError, observations::Observation, Birders, Location};

use super::RecentNearbyNotableParams;

pub struct RecentNearbyNotableHandler<'birder> {
    birder: &'birder Birders,
    latitude: f64,
    longitude: f64,
    params: Option<RecentNearbyNotableParams>,
}

impl<'birder> RecentNearbyNotableHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        location: &Location,
        params: Option<RecentNearbyNotableParams>,
    ) -> Self {
        Self {
            birder,
            params,
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }
}

impl<'birder> RecentNearbyNotableHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/obs/geo/recent/notable?lat={}&lng={}",
            self.latitude, self.longitude
        );

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
