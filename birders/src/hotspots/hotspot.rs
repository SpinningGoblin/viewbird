use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Hotspot {
    #[serde(alias = "locId")]
    pub loc_id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(alias = "countryCode")]
    pub country_code: String,
    #[serde(alias = "countryName")]
    pub country_name: String,
    #[serde(alias = "subnational1Code")]
    pub subnational_1_code: String,
    #[serde(alias = "subnational2Code")]
    pub subnational_2_code: String,
    #[serde(alias = "subnational1Name")]
    pub subnational_1_name: String,
    #[serde(alias = "subnational2Name")]
    pub subnational_2_name: String,
    #[serde(alias = "isHotspot")]
    pub is_hotspot: bool,
    #[serde(alias = "locName")]
    pub loc_name: String,
    // Yes, hotspots have both latitude, longitude, lat and lng.
    pub lat: f64,
    pub lng: f64,
    #[serde(alias = "hierarchicalName")]
    pub hierarchical_name: String,
}
