use rand::prelude::*;
use std::fmt::Error;

use crate::{config::Config, judge::Judge, project::Project};

pub trait Allocator {
  fn allocate(&self) -> Result<Allocations, Error>;
}

// Allocation for a single judge and their assigned projects.
pub struct Allocation {
  // Judge that has projects allocated to it.
  pub judge: Judge,
  // Projects allocated to the judge.
  pub projects: Vec<Project>,
}

impl Allocation {
  pub fn new(judge: Judge, projects: Vec<Project>) -> Self {
    Allocation { judge, projects }
  }
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

impl std::fmt::Debug for Allocations {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for allocation in &self.allocations {
      writeln!(f, "Judge: {}", allocation.judge.name)?;
      for project in &allocation.projects {
        writeln!(f, "  Project: {}", project.name)?;
      }
    }
    Ok(())
  }
}

impl Allocations {
  pub fn new(allocations: Vec<Allocation>) -> Self {
    Allocations { allocations }
  }
}

pub struct RandomAllocator {
  config: Config,
  judges: Vec<Judge>,
  projects: Vec<Project>,
}

impl RandomAllocator {
  pub fn new(config: Config, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    RandomAllocator {
      config,
      judges,
      projects,
    }
  }
}

impl Allocator for RandomAllocator {
  fn allocate(&self) -> Result<Allocations, Error> {
    let mut allocations: Vec<Allocation> = Vec::new();

    for judge in &self.judges {
      allocations.push(Allocation::new(judge.clone(), Vec::new()));
    }

    for project in &self.projects {
      let mut judges_allocated = 0;
      while judges_allocated < self.config.judge_amount {
        let idx = rand::rng().random_range(0..allocations.len());
        let allocation = allocations.get_mut(idx).unwrap();

        if allocation.projects.contains(&project) {
          continue;
        }

        allocation.projects.push(project.clone());
        judges_allocated += 1;
      }
    }

    Ok(Allocations::new(allocations))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::config::Config;

  #[test]
  fn test_random_allocator() {
    let config = Config {
      judge_amount: 2,
      ..Default::default()
    };

    let judges = vec![
      Judge {
        id: "1".to_string(),
        name: "Judge 1".to_string(),
      },
      Judge {
        id: "2".to_string(),
        name: "Judge 2".to_string(),
      },
    ];

    let projects = vec![
      Project {
        id: "1".to_string(),
        name: "Project 1".to_string(),
      },
      Project {
        id: "2".to_string(),
        name: "Project 2".to_string(),
      },
    ];

    let allocator = RandomAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate().unwrap();

    for allocation in &allocations.allocations {
      assert_eq!(allocation.projects.len(), 2);
    }
  }
}
