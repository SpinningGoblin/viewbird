#[derive(Default)]
pub struct EbirdTaxonomyParams {
    /// Only fetch observations from the given taxonomic category.
    /// Defaults to all in ebird API.
    pub cat: Option<String>,
    /// What locale to use for the common name,
    /// Defaults to en in the ebird API.
    pub locale: Option<String>,
    /// The different species codes to filter the taxonomies returned to.
    pub species: Option<Vec<String>>,
    /// Fetch a specific version of the taxonomy.
    pub version: Option<String>,
}

impl EbirdTaxonomyParams {
    pub fn to_url(&self) -> Vec<String> {
        let mut url_params = Vec::new();

        if let Some(cat) = &self.cat {
            url_params.push(format!("cat={cat}"));
        }

        if let Some(locale) = &self.locale {
            url_params.push(format!("locale={locale}"));
        }

        if let Some(species) = &self.species {
            url_params.push(format!("species={}", species.join(",")));
        }

        if let Some(version) = &self.version {
            url_params.push(format!("version={version}"));
        }

        url_params
    }
}
