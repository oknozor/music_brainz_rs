use core::marker::PhantomData;

use crate::client::MusicBrainzClient;
use crate::entity::Include;
use crate::FMT_JSON;
use crate::PARAM_INC;

pub mod browse;
pub mod relations;

/// The base element of a query
#[derive(Clone, Debug)]
pub(crate) struct Query<T> {
    /// The path of the api to query
    pub(crate) path: String,

    /// The includes added to the query
    pub(crate) include: Vec<Include>,

    /// The resulting type of the query
    pub(crate) result_type: PhantomData<T>,
}

impl<T> Query<T> {
    /// Add an include parameter to the query
    pub(crate) fn include(&mut self, include: Include) -> &mut Self {
        self.include.push(include);
        self
    }

    /// Create the full url path of the query
    pub(crate) fn create_url(&self, client: &MusicBrainzClient) -> String {
        let mut url = format!("{}/{}{}", client.musicbrainz_url, self.path, FMT_JSON);

        // If we don't have includes, let's return early
        if self.include.is_empty() {
            return url;
        }

        url.push_str(PARAM_INC);

        for inc in &self.include {
            url.push_str(inc.as_str());
            if Some(inc) != self.include.last() {
                url.push('+');
            }
        }

        url
    }
}