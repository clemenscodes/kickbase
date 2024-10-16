use reqwest::Method;
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_feed(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/players/{}/feed", league_id, player_id);
    let response: HttpResponse<Value> = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}
