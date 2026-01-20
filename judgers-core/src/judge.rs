use serde::{Deserialize, Serialize};

use crate::{error::Error, Validate};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Judge {
  /// Id of the judge, this value must be unique.
  pub id: String,
  /// Name of the judge.
  pub name: String,
}

impl Judge {
  pub fn new(id: String, name: String) -> Self {
    Judge { id, name }
  }
}

impl PartialEq for Judge {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.name == other.name
  }
}

impl Eq for Judge {}

impl std::fmt::Display for Judge {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.id)
  }
}

impl Validate for Judge {
  fn validate(&self) -> Result<(), Error> {
    if self.id.trim().is_empty() {
      return Err(Error::ErrInvalidJudgeId(self.id.clone()));
    }

    if self.name.trim().is_empty() {
      return Err(Error::ErrInvalidJudgeName(self.name.clone()));
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_judge_validation_valid() {
    let judge = Judge::new("j1".to_string(), "Judge One".to_string());

    assert!(judge.validate().is_ok());
  }

  #[test]
  fn test_judge_validation_invalid_id() {
    let judge = Judge::new("".to_string(), "Judge One".to_string());

    assert_eq!(judge.validate(), Err(Error::ErrInvalidJudgeId("".to_string())));
  }

  #[test]
  fn test_judge_validation_invalid_name() {
    let judge = Judge::new("j1".to_string(), "".to_string());

    assert_eq!(judge.validate(), Err(Error::ErrInvalidJudgeName("".to_string())));
  }
}
