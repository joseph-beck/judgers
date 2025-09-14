#[derive(Clone)]
pub struct Project {
  // Id of the project, this value must be unique.
  pub id: String,
  // Name of the project.
  pub name: String,
}

impl PartialEq for Project {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.name == other.name
  }
}

impl Eq for Project {}

impl Project {
  pub fn new(id: String, name: String) -> Self {
    Project { id, name }
  }
}
