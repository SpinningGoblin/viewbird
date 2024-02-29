use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum RegionType {
    #[serde(alias = "country")]
    Country,
    #[serde(alias = "subnational1")]
    Subnational1,
    #[serde(alias = "subnational2")]
    Subnational2,
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RegionType::Country => "country",
            RegionType::Subnational1 => "subnational1",
            RegionType::Subnational2 => "subnational2",
        };

        f.write_str(value)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::RegionType;

    #[derive(Deserialize)]
    struct TestStruct {
        region: RegionType,
    }

    #[test]
    pub fn deserialize() {
        let serialized = r#"
            {
              "region": "subnational1"
            }
            "#;
        let deserialized: TestStruct = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized.region, RegionType::Subnational1);
    }
}
