use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bounds {
    #[serde(alias = "minX")]
    pub min_x: f64,
    #[serde(alias = "maxX")]
    pub max_x: f64,
    #[serde(alias = "minY")]
    pub min_y: f64,
    #[serde(alias = "maxY")]
    pub max_y: f64,
}

#[cfg(test)]
mod tests {
    use super::Bounds;

    #[test]
    pub fn deserialize_alias_fields() {
        let serialized = r#"
            {
              "minX": -124.500551,
              "maxX": -123.044273,
              "minY": 48.308613,
              "maxY": 49.013333
            }
            "#;
        let deserialized: Bounds = serde_json::from_str(serialized).unwrap();
        assert_eq!(deserialized.min_x, -124.500551);
        assert_eq!(deserialized.max_x, -123.044273);
        assert_eq!(deserialized.min_y, 48.308613);
        assert_eq!(deserialized.max_y, 49.013333);
    }
}
