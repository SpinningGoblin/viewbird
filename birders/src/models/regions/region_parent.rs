use serde::{Deserialize, Serialize};

use super::RegionType;

/// Region that comes back as a parent of another region when
/// getting the info of the region.
/// Do not want to use the same type as Region, so as to not
/// create a circular reference.
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
