use crate::{errors::BirderError, taxonomies::Taxonomy, Birders};

use super::EbirdTaxonomyParams;

pub struct EbirdTaxonomyHandler<'birder> {
    birder: &'birder Birders,
    params: Option<EbirdTaxonomyParams>,
}

impl<'birder> EbirdTaxonomyHandler<'birder> {
    pub fn new(birder: &'birder Birders, params: Option<EbirdTaxonomyParams>) -> Self {
        Self { birder, params }
    }
}

impl<'birder> EbirdTaxonomyHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Taxonomy>, BirderError> {
        let url = "/ref/taxonomy/ebird?fmt=json";

        let full_url = match &self.params {
            Some(it) => format!("{}&{}", url, it.to_url().join("&")),
            None => url.to_string(),
        };

        self.birder.get(&full_url).await
    }
}
