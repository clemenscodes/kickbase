use reqwest::Method;

use super::{HttpClient, HttpClientError};

#[derive(Debug)]
pub struct League {
  pub id: String,
  pub name: String,
  pub creator: String,
  pub creation: String,
  pub image: String,
}

impl HttpClient {
  pub async fn get_leagues(&self) -> Result<Vec<League>, HttpClientError> {
    let response = self.get(Method::GET, "/leagues", None).await.unwrap();

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

    Ok(leagues)
  }
}
