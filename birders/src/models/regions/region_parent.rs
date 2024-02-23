use serde::{Deserialize, Serialize};

use super::RegionType;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RegionParent {
    pub result: String,
    pub code: String,
    #[serde(alias = "type")]
    pub region_type: RegionType,
    pub latitude: f64,
    pub longitude: f64,
    pub parent: Option<Box<RegionParent>>,
}
