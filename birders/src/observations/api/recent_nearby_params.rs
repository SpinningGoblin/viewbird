use std::fmt::Display;

/// Optional parameters for recent observations in a region.
#[derive(Default)]
pub struct RecentNearbyParams {
    /// How many days back to look up data for. Max is 30.
    /// Will default to None if not provided.
    pub back: Option<u8>,
    /// Only fetch observations from the given taxonomic category.
    /// Defaults to all in ebird API.
    pub cat: Option<String>,
    /// Whether to only return observations from hotspots.
    /// Defaults to false in ebird API.
    pub hotspot: Option<bool>,
    /// Include observations which have not been reviewed.
    /// Defaults to false in ebird API.
    pub include_provisional: Option<bool>,
    /// How many results to return. Range is 1-10_000
    /// Defaults to returning all in ebird API.
    pub max_results: Option<u16>,
    /// What locale to use for the common name,
    /// Defaults to en in the ebird API.
    pub app_locale: Option<String>,
    /// Sort by taxonomoy or by date, with most recent observations first.
    pub sort: Option<NearbySortType>,
    /// The search radius from the given position, in kilometers.
    pub dist: Option<u8>,
}

pub enum NearbySortType {
    Date,
    Species,
}

impl Display for NearbySortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            NearbySortType::Date => "date",
            NearbySortType::Species => "species",
        };

        f.write_str(value)
    }
}

impl RecentNearbyParams {
    pub fn to_url(&self) -> Vec<String> {
        let mut url_params = Vec::new();

        if let Some(back) = self.back {
            url_params.push(format!("back={back}"));
        }

        if let Some(cat) = &self.cat {
            url_params.push(format!("cat={cat}"));
        }

        if let Some(hotspot) = self.hotspot {
            url_params.push(format!("hotspot={hotspot}"));
        }

        if let Some(include_provisional) = self.include_provisional {
            url_params.push(format!("includeProvisional={include_provisional}"));
        }

        if let Some(max_results) = self.max_results {
            url_params.push(format!("maxResults={max_results}"));
        }

        if let Some(sort) = &self.sort {
            url_params.push(format!("sort={sort}"));
        }

        if let Some(app_locale) = &self.app_locale {
            url_params.push(format!("appLocale={app_locale}"));
        }

        if let Some(dist) = &self.dist {
            url_params.push(format!("dist={dist}"));
        }

        url_params
    }
}
