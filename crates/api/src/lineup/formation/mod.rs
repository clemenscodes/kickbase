#[derive(Debug, Clone, Copy)]
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
