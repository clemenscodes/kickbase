use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_feed_comments(
    &self,
    league_id: &str,
    feed_item_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/feed/{feed_item_id}/comments");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
