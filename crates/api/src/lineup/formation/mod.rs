use std::fmt::Debug;

use serde_json::Value;

#[derive(Clone, Copy)]
pub enum Formation {
  ThreeFourThree,
  ThreeFiveTwo,
  ThreeSixOne,
  FourTwoFour,
  FourThreeThree,
  FourFourTwo,
  FourFiveOne,
  FiveTwoThree,
  FiveThreeTwo,
  FiveFourOne,
  Invalid,
}

impl From<&Value> for Formation {
  fn from(value: &Value) -> Self {
    Self::from(value.get("type").unwrap().as_str().unwrap())
  }
}

impl From<&str> for Formation {
  fn from(value: &str) -> Self {
    match value {
      "3-4-3" => Self::ThreeFourThree,
      "3-5-2" => Self::ThreeFiveTwo,
      "3-6-1" => Self::ThreeSixOne,
      "4-2-4" => Self::FourTwoFour,
      "4-3-3" => Self::FourThreeThree,
      "4-4-2" => Self::FourFourTwo,
      "4-5-1" => Self::FourFiveOne,
      "5-2-3" => Self::FiveTwoThree,
      "5-3-2" => Self::FiveThreeTwo,
      "5-4-1" => Self::FiveFourOne,
      _ => Self::Invalid,
    }
  }
}

impl From<String> for Formation {
  fn from(value: String) -> Self {
    Self::from(value.as_str())
  }
}

impl std::fmt::Display for Formation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let formation_str = match self {
      Formation::ThreeFourThree => "3-4-3",
      Formation::ThreeFiveTwo => "3-5-2",
      Formation::ThreeSixOne => "3-6-1",
      Formation::FourTwoFour => "4-2-4",
      Formation::FourThreeThree => "4-3-3",
      Formation::FourFourTwo => "4-4-2",
      Formation::FourFiveOne => "4-5-1",
      Formation::FiveTwoThree => "5-2-3",
      Formation::FiveThreeTwo => "5-3-2",
      Formation::FiveFourOne => "5-4-1",
      Formation::Invalid => "Invalid",
    };
    write!(f, "{}", formation_str)
  }
}

impl Debug for Formation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
