use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPlayerToMarketPayload {
  #[serde(rename = "playerId")]
  pub player_id: String,
  pub price: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePricePayload {
  pub price: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOfferPayload {
  pub price: u64,
}

impl HttpClient {
  pub async fn remove_player_from_market(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/market/{player_id}");
    let response = self.delete(&url).await?;
    Ok(response)
  }

  pub async fn get_market(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/market");
    let response = self.get(&url).await?;
    Ok(response)
  }

  pub async fn add_player_to_market(
    &self,
    league_id: &str,
    payload: AddPlayerToMarketPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/market");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }

  pub async fn accept_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!(
      "/leagues/{league_id}/market/{player_id}/offers/{offer_id}/accept"
    );
    let response = self.post(&url).await?;
    Ok(response)
  }

  pub async fn update_price(
    &self,
    league_id: &str,
    player_id: &str,
    payload: UpdatePricePayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/market/{player_id}");
    let response = self.req(Method::PUT, &url, &payload).await?;
    Ok(response)
  }

  pub async fn decline_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!(
      "/leagues/{league_id}/market/{player_id}/offers/{offer_id}/decline"
    );
    let response = self.post(&url).await?;
    Ok(response)
  }

  pub async fn place_offer(
    &self,
    league_id: &str,
    player_id: &str,
    payload: PlaceOfferPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/market/{player_id}/offers");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }

  pub async fn remove_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url =
      format!("/leagues/{league_id}/market/{player_id}/offers/{offer_id}");
    let response = self.delete(&url).await?;
    Ok(response)
  }
}
