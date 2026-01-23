use serde::{Deserialize, Serialize};

use crate::{error::Error, Validate};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
  /// Id of the project, this value must be unique.
  pub id: String,
  /// Name of the project.
  pub name: String,
  /// Table number assigned to the project (optional).
  pub table_number: Option<u32>,
}

impl Project {
  pub fn new(id: String, name: String) -> Self {
    Project {
      id,
      name,
      table_number: None,
    }
  }

  pub fn with_table_number(mut self, table_number: u32) -> Self {
    self.table_number = Some(table_number);

    self
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

impl Validate for Project {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_project_validation_with_table_number() {
    let project = Project::new("p1".to_string(), "Project One".to_string()).with_table_number(5);

    assert!(project.validate().is_ok());
    assert_eq!(project.table_number, Some(5));
  }

  #[test]
  fn test_project_validation_valid() {
    let project = Project::new("p1".to_string(), "Project One".to_string());

    assert!(project.validate().is_ok());
  }

  #[test]
  fn test_project_validation_invalid_id() {
    let project = Project::new("".to_string(), "Project One".to_string());

    assert_eq!(project.validate(), Err(Error::ErrInvalidProjectId("".to_string())));
  }

  #[test]
  fn test_project_validation_invalid_name() {
    let project = Project::new("p1".to_string(), "".to_string());

    assert_eq!(project.validate(), Err(Error::ErrInvalidProjectName("".to_string())));
  }
}
