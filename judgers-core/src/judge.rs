#[derive(Clone)]
pub struct Judge {
  // Id of the judge, this value must be unique.
  pub id: String,
  // Name of the judge.
  pub name: String,
}

impl Judge {
  pub fn new(id: String, name: String) -> Self {
    Judge { id, name }
  }
}
