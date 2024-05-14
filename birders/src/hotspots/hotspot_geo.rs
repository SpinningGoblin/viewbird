use serde::{Deserialize, Serialize};

/// Response from the hotspot/geo endpoint.
/// Geographic information and quick count for a hotspot.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HotspotGeo {
    #[serde(alias = "locId")]
    pub loc_id: String,
    #[serde(alias = "locName")]
    pub loc_name: String,
    #[serde(alias = "countryCode")]
    pub country_code: String,
    #[serde(alias = "subnational1Code")]
    pub subnational_1_code: String,
    #[serde(alias = "subnational2Code")]
    pub subnational_2_code: String,
    pub lat: f64,
    pub lng: f64,
    #[serde(alias = "latestObsDt")]
    #[serde(default)]
    pub latest_obs_dt: Option<String>,
    #[serde(alias = "numSpeciesAllTime")]
    pub num_species_all_time: u64,
}
