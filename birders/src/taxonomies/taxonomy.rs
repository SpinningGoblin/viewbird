use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Taxonomy {
    #[serde(alias = "sciName")]
    pub sci_name: String,
    #[serde(alias = "comName")]
    pub com_name: String,
    #[serde(alias = "speciesCode")]
    pub species_code: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(alias = "taxonOrder")]
    pub taxon_order: f64,
    #[serde(alias = "bandingCodes")]
    #[serde(default)]
    pub banding_codes: Vec<String>,
    #[serde(alias = "comNameCodes")]
    #[serde(default)]
    pub com_name_codes: Vec<String>,
    #[serde(alias = "sciNameCodes")]
    #[serde(default)]
    pub sci_name_codes: Vec<String>,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(alias = "familyCode")]
    #[serde(default)]
    pub family_code: Option<String>,
    #[serde(alias = "familyComName")]
    #[serde(default)]
    pub family_com_name: Option<String>,
    #[serde(alias = "familySciName")]
    #[serde(default)]
    pub family_sci_name: Option<String>,
    #[serde(alias = "reportAs")]
    #[serde(default)]
    pub report_as: Option<String>,
}
