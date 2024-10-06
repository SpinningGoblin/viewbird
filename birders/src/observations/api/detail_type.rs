use std::fmt::Display;

pub enum DetailType {
    Simple,
    Full,
}

impl Display for DetailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DetailType::Simple => "simple",
            DetailType::Full => "full",
        };

        f.write_str(value)
    }
}
