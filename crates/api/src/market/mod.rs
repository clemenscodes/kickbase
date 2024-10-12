use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct AddPlayerToMarketPayload {
  pub player_id: String,
  pub price: u64,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePricePayload {
  pub price: u64,
}

impl HttpClient {
  pub async fn remove_player_from_market(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/market/{}", league_id, player_id);
    let response = self.get(Method::DELETE, &url, None).await?;
    Ok(response)
  }

  pub async fn get_market(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/market", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }

  pub async fn add_player_to_market(
    &self,
    league_id: &str,
    payload: AddPlayerToMarketPayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("playerId", payload.player_id);
    map.insert("price", payload.price.to_string());

    let url = format!("/leagues/{}/market", league_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }

  pub async fn accept_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!(
      "/leagues/{}/market/{}/offers/{}/accept",
      league_id, player_id, offer_id
    );
    let response = self.get(Method::POST, &url, None).await?;
    Ok(response)
  }

  pub async fn update_price(
    &self,
    league_id: &str,
    player_id: &str,
    payload: UpdatePricePayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("price", payload.price.to_string());

    let url = format!("/leagues/{}/market/{}", league_id, player_id);
    let response = self.req(Method::PUT, &url, Some(&map), None).await?;
    Ok(response)
  }

  pub async fn decline_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!(
      "/leagues/{}/market/{}/offers/{}/decline",
      league_id, player_id, offer_id
    );
    let response = self.get(Method::POST, &url, None).await?;
    Ok(response)
  }

  pub async fn place_offer(
    &self,
    league_id: &str,
    player_id: &str,
    price: u64,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("price", price.to_string());

    let url = format!("/leagues/{}/market/{}/offers", league_id, player_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }

  pub async fn remove_offer(
    &self,
    league_id: &str,
    player_id: &str,
    offer_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!(
      "/leagues/{}/market/{}/offers/{}",
      league_id, player_id, offer_id
    );
    let response = self.get(Method::DELETE, &url, None).await?;
    Ok(response)
  }
}
