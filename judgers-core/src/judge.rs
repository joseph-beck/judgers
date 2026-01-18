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
