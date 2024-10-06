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
    #[serde(default)]
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

    // Fields from "full" follow
    #[serde(alias = "subnational2Code")]
    #[serde(default)]
    pub subnational_2_code: Option<String>,
    #[serde(alias = "subnational2Name")]
    #[serde(default)]
    pub subnational_2_name: Option<String>,
    #[serde(alias = "subnational1Code")]
    #[serde(default)]
    pub subnational_1_code: Option<String>,
    #[serde(alias = "subnational1Name")]
    #[serde(default)]
    pub subnational_1_name: Option<String>,
    #[serde(alias = "countryCode")]
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(alias = "countryName")]
    #[serde(default)]
    pub country_name: Option<String>,
    #[serde(alias = "userDisplayName")]
    #[serde(default)]
    pub user_display_name: Option<String>,
    #[serde(alias = "obsId")]
    #[serde(default)]
    pub obs_id: Option<String>,
    #[serde(alias = "checkListId")]
    #[serde(default)]
    pub check_list_id: Option<String>,
    #[serde(alias = "presenceNoted")]
    #[serde(default)]
    pub presence_noted: Option<bool>,
    #[serde(alias = "hasComments")]
    #[serde(default)]
    pub has_comments: Option<bool>,
    #[serde(alias = "firstName")]
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(alias = "lastName")]
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(alias = "hasRichMedia")]
    #[serde(default)]
    pub has_rich_media: Option<bool>,
}
