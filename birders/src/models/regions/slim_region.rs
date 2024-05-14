use serde::{Deserialize, Serialize};

/// A region returned when listing the regions in another region.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SlimRegion {
    pub code: String,
    pub name: String,
}
