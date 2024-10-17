use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{HttpClient, HttpClientError};
use crate::HttpResponse;

impl HttpClient {
  pub async fn comment_league(
    &self,
    league_id: &str,
    payload: CommentLeaguePayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/comment");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentLeaguePayload {
  comment: String,
}
