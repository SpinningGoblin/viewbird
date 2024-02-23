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
