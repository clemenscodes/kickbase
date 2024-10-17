use crate::{HttpClient, HttpClientError, HttpResponse};
use serde_json::Value;

impl HttpClient {
  pub async fn get_messages(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/chat/messages");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
