use axum::http::HeaderMap;
use reqwest::Method;

use crate::HTTP;

use super::HttpClient;

#[derive(Debug)]
pub struct League {
  pub id: String,
  pub name: String,
  pub creator: String,
  pub creation: String,
  pub image: String,
}

impl HttpClient {
  pub async fn get_leagues(&self, headers: Option<HeaderMap>) -> Vec<League> {
    let response = HTTP
      .read()
      .await
      .get(Method::GET, "/leagues", headers)
      .await
      .unwrap();

    let leagues: Vec<League> = response
      .value
      .get("leagues")
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(|league| {
        let name = league.get("name").unwrap().to_string().replace("\"", "");
        let id = league.get("id").unwrap().to_string().replace("\"", "");
        let creator =
          league.get("creator").unwrap().to_string().replace("\"", "");
        let creation = league
          .get("creation")
          .unwrap()
          .to_string()
          .replace("\"", "");
        let image = league.get("ci").unwrap().to_string().replace("\"", "");
        League {
          id,
          name,
          creator,
          creation,
          image,
        }
      })
      .collect();

    leagues
  }
}
