use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_user_profile(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/users/{user_id}");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
