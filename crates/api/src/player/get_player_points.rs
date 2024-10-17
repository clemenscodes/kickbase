use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_points(
    &self,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/players/{player_id}/points");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
