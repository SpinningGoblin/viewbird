use std::fmt::Display;

pub enum Rank {
    Mrec,
    Create,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Rank::Mrec => "mrec",
            Rank::Create => "create",
        };

        f.write_str(value)
    }
}
