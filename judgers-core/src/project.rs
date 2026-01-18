use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
  /// Id of the project, this value must be unique.
  pub id: String,
  /// Name of the project.
  pub name: String,
}

impl Project {
  pub fn new(id: String, name: String) -> Self {
    Project { id, name }
  }
}

impl PartialEq for Project {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.name == other.name
  }
}

impl Eq for Project {}

impl std::fmt::Display for Project {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} ({})", self.name, self.id)
  }
}

impl crate::Validate for Project {
  fn validate(&self) -> Result<(), Error> {
    if self.id.trim().is_empty() {
      return Err(Error::ErrInvalidProjectId(self.id.clone()));
    }

    if self.name.trim().is_empty() {
      return Err(Error::ErrInvalidProjectName(self.name.clone()));
    }

    Ok(())
  }
}
