use super::*;
use crate::structures::{search::*, Number};

impl Ferinth {
    /// Search for projects using `query` string, with pagination
    ///
    /// Limit the number of responses to `limit` projects, and offset the output by `offset` projects.
    /// Sort mods by `sort`, and filter mods using the given `facets`.
    pub async fn search_paged(
        &self,
        query: &str,
        sort: &Sort,
        limit: &Number,
        offset: &Number,
        facets: &[&[Facet]],
    ) -> Result<Response> {
        self.client
            .get(
                API_BASE_URL.join_all(vec!["search"])
                    .with_query_json("query", query)?
                    .with_query_json("index", &sort.to_string())?
                    .with_query_json("limit", &limit.to_string())?
                    .with_query_json("offset", &offset.to_string())?
                    .with_query_json("facets", &serde_json::to_string(facets)?)?,
            )
            .custom_send_json()
            .await
    }

    /// Search for projects using `query` string
    ///
    /// Sort mods by `sort`, and filter mods using the given `facets`.
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::search::Sort;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let results = modrinth.search("sodium", &Sort::Relevance, &[]).await?;
    /// // The Sodium mod should be the most relevant response when `"sodium"` is searched
    /// assert_eq!(&results.hits[0].id, "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn search(&self, query: &str, sort: &Sort, facets: &[&[Facet]]) -> Result<Response> {
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["search"])
                    .with_query("query", query)
                    .with_query("index", &sort.to_string())
                    .with_query("facets", &serde_json::to_string(facets)?),
            )
            .custom_send_json()
            .await
    }
}
