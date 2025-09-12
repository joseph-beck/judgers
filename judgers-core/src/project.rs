#[derive(Clone)]
pub struct Project {
  pub id: String,
  pub name: String,
}

impl PartialEq for Project {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.name == other.name
  }
}

impl Eq for Project {}
