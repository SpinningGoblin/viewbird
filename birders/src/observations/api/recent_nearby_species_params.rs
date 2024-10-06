/// Optional parameters for recent observations in a region.
#[derive(Default)]
pub struct RecentNearbySpeciesParams {
    /// How many days back to look up data for. Max is 30.
    /// Will default to None if not provided.
    pub back: Option<u8>,
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
    /// The search radius from the given position, in kilometers.
    pub dist: Option<u8>,
}

impl RecentNearbySpeciesParams {
    pub fn to_url(&self) -> Vec<String> {
        let mut url_params = Vec::new();

        if let Some(back) = self.back {
            url_params.push(format!("back={back}"));
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

        if let Some(app_locale) = &self.app_locale {
            url_params.push(format!("appLocale={app_locale}"));
        }

        if let Some(dist) = &self.dist {
            url_params.push(format!("dist={dist}"));
        }

        url_params
    }
}
