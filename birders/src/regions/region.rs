use serde::{Deserialize, Serialize};

use super::{Bounds, RegionParent, RegionType};

/// A geographic region such as a country like Canada, or a
/// province like British Columbia.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Region {
    pub bounds: Bounds,
    pub result: String,
    pub code: String,
    #[serde(alias = "type")]
    pub region_type: RegionType,
    pub longitude: f64,
    pub latitude: f64,
    pub parent: Option<RegionParent>,
}

#[cfg(test)]
mod tests {

    use crate::regions::{Bounds, RegionParent, RegionType};

    use super::Region;

    #[test]
    pub fn deserialize() {
        let serialized = r#"
        {
          "bounds": {
            "minX": -124.500551,
            "maxX": -123.044273,
            "minY": 48.308613,
            "maxY": 49.013333
          },
        "result": "Capital, British Columbia, Canada",
        "code": "CA-BC-CP",
        "type": "subnational2",
        "parent": {
          "result": "British Columbia, Canada",
          "code": "CA-BC",
          "type": "subnational1",
          "parent": {
            "result": "Canada",
            "code": "CA",
            "type": "country",
            "longitude": 0.0,
            "latitude": 0.0
          },
          "longitude": 0.0,
          "latitude": 0.0
        },
        "longitude": 48.660973,
        "latitude": -123.772412
        }
        "#;

        let deserialized: Region = serde_json::from_str(serialized).unwrap();
        let expected: Region = Region {
            bounds: Bounds {
                min_x: -124.500551,
                max_x: -123.044273,
                min_y: 48.308613,
                max_y: 49.013333,
            },
            result: "Capital, British Columbia, Canada".to_string(),
            code: "CA-BC-CP".to_string(),
            region_type: RegionType::Subnational2,
            longitude: 48.660973,
            latitude: -123.772412,
            parent: Some(RegionParent {
                result: "British Columbia, Canada".to_string(),
                code: "CA-BC".to_string(),
                region_type: RegionType::Subnational1,
                latitude: 0.0,
                longitude: 0.0,
                parent: Some(Box::new(RegionParent {
                    result: "Canada".to_string(),
                    code: "CA".to_string(),
                    region_type: RegionType::Country,
                    latitude: 0.0,
                    longitude: 0.0,
                    parent: None,
                })),
            }),
        };

        assert_eq!(deserialized, expected);
    }
}
