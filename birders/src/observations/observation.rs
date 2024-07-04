use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Observation {
    #[serde(alias = "speciesCode")]
    pub species_code: String,
    #[serde(alias = "comName")]
    pub com_name: String,
    #[serde(alias = "sciName")]
    pub sci_name: String,
    #[serde(alias = "locId")]
    pub loc_id: String,
    #[serde(alias = "locName")]
    pub loc_name: String,
    #[serde(alias = "obsDt")]
    pub obs_dt: String,
    #[serde(alias = "howMany")]
    pub how_many: u64,
    #[serde(alias = "lat")]
    pub latitude: f64,
    #[serde(alias = "lng")]
    pub longitude: f64,
    #[serde(alias = "obsValid")]
    pub obs_valid: bool,
    #[serde(alias = "obsReviewed")]
    pub obs_reviewed: bool,
    #[serde(alias = "locationPrivate")]
    pub location_private: bool,
    #[serde(alias = "subId")]
    pub sub_id: String,
    #[serde(alias = "exoticCategory")]
    #[serde(default)]
    pub exotic_category: Option<String>,
}
