use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SlimRegion {
    pub code: String,
    pub name: String,
}
