/// Optional parameters for recent observations in a region.
#[derive(Default)]
pub struct RecentInRegionParams {
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
    /// List of up to 10 region codes to include observations for.
    /// Defaults to None in ebird API which will just return from overall region asked for.
    pub r: Option<Vec<String>>,
    /// What locale to use for the common name,
    /// Defaults to en in the ebird API.
    pub app_locale: Option<String>,
}

impl RecentInRegionParams {
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

        if let Some(r) = &self.r {
            url_params.push(format!("r={}", r.join(",")));
        }

        if let Some(app_locale) = &self.app_locale {
            url_params.push(format!("appLocale={app_locale}"));
        }

        url_params
    }
}
