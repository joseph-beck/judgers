use std::fmt::Error;

use crate::{judge::Judge, project::Project};

pub trait Allocator {
  fn allocate(&self) -> Result<Error, Allocations>;
}

// Allocation for a single judge and their assigned projects.
pub struct Allocation {
  // Judge that has projects allocated to it.
  pub judge: Judge,
  // Projects allocated to the judge.
  pub projects: Vec<Project>,
}

// Allocations for all judges and projects.
pub struct Allocations {
  // Vec of all allocations.
  // Defaults to empty vec.
  pub allocations: Vec<Allocation>,
}

impl Default for Allocations {
  fn default() -> Self {
    Allocations {
      allocations: Vec::new(),
    }
  }
}
