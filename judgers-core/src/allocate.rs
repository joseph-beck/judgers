use rand::prelude::*;

use crate::{config::Config, error, judge::Judge, project::Project};

/// Allocator trait, must be implemented by all allocators.
pub trait Allocator {
  /// Allocate, returns a mapping of judges to projects.
  /// May return an error if allocation is not possible.
  fn allocate(&self) -> Result<Allocations, error::Error>;
}

/// Allocation for a single judge and their assigned projects.
pub struct Allocation {
  /// Judge that has projects allocated to it.
  pub judge: Judge,
  /// Projects allocated to the judge.
  pub projects: Vec<Project>,
}

impl Allocation {
  pub fn new(judge: Judge, projects: Vec<Project>) -> Self {
    Allocation { judge, projects }
  }
}

/// Allocations for all judges and projects.
pub struct Allocations {
  /// Vec of all allocations.
  /// Defaults to empty vec.
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

/// A random, science fair style, allocator.
/// It is not guaranteed that each judge will view every project.
/// Each project will be judged by a unique judge.
pub struct RandomFairAllocator {
  /// General configuration for allocators.
  config: Config,
  /// All judges that are used for allocations.
  judges: Vec<Judge>,
  /// All projects that will be assigned to judges.
  projects: Vec<Project>,
}

impl RandomFairAllocator {
  pub fn new(config: Config, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    RandomFairAllocator {
      config,
      judges,
      projects,
    }
  }
}

impl Allocator for RandomFairAllocator {
  fn allocate(&self) -> Result<Allocations, error::Error> {
    let mut allocations: Vec<Allocation> = Vec::new();

    for judge in &self.judges {
      allocations.push(Allocation::new(judge.clone(), Vec::new()));
    }

    if self.config.judge_amount > self.judges.len() as u32 {
      return Err(error::Error::NotEnoughJudges {
        judges: self.judges.len() as u32,
        projects: self.projects.len() as u32,
        judge_amount: self.config.judge_amount,
      });
    }

    for project in &self.projects {
      let mut judges_allocated = 0;
      while judges_allocated < self.config.judge_amount {
        let index = rand::rng().random_range(0..allocations.len());
        let allocation = allocations.get_mut(index).unwrap();

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

/// Presentation style allocator.
/// Each judge will see every project, typically at the same time.
pub struct PresentationAllocator {
  /// Config for the allocator.
  /// Judge amount will be ignored for this allocator.
  _config: Config,
  /// All judges that are used for allocations.
  judges: Vec<Judge>,
  /// All projects that will be assigned to judges.
  projects: Vec<Project>,
}

impl PresentationAllocator {
  pub fn new(_config: Config, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    PresentationAllocator {
      _config,
      judges,
      projects,
    }
  }
}

impl Allocator for PresentationAllocator {
  fn allocate(&self) -> Result<Allocations, error::Error> {
    let mut allocations: Vec<Allocation> = Vec::new();

    for judge in &self.judges {
      allocations.push(Allocation::new(judge.clone(), Vec::new()));
    }

    for allocation in &mut allocations {
      allocation.projects = self.projects.clone();
    }

    Ok(Allocations::new(allocations))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::config::Config;
  use std::collections::HashMap;

  #[test]
  fn test_random_allocator_with_two() {
    let config = Config {
      judge_amount: 2,
      ..Default::default()
    };

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
    ];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
      Project::new("4".to_string(), "Project 4".to_string()),
    ];

    let allocator = RandomFairAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate().unwrap();

    let mut project_counts: HashMap<String, usize> = HashMap::new();

    for allocation in &allocations.allocations {
      for project in &allocation.projects {
        *project_counts.entry(project.id.clone()).or_insert(0) += 1;
      }
    }

    for project in &projects {
      assert_eq!(
        project_counts.get(&project.id),
        Some(&2),
        "Project {} was not allocated exactly twice",
        project.name
      );
    }
  }

  #[test]
  fn test_random_allocator_with_three() {
    let config = Config {
      judge_amount: 3,
      ..Default::default()
    };

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
      Judge::new("3".to_string(), "Judge 3".to_string()),
    ];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
      Project::new("4".to_string(), "Project 4".to_string()),
      Project::new("5".to_string(), "Project 5".to_string()),
      Project::new("6".to_string(), "Project 6".to_string()),
    ];

    let allocator = RandomFairAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate().unwrap();

    let mut project_counts: HashMap<String, usize> = HashMap::new();

    for allocation in &allocations.allocations {
      for project in &allocation.projects {
        *project_counts.entry(project.id.clone()).or_insert(0) += 1;
      }
    }

    for project in &projects {
      assert_eq!(
        project_counts.get(&project.id),
        Some(&3),
        "Project {} was not allocated exactly twice",
        project.name
      );
    }
  }

  #[test]
  fn test_random_allocator_error_not_enough_judges() {
    let config = Config {
      judge_amount: 3,
      ..Default::default()
    };

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
    ];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
      Project::new("4".to_string(), "Project 4".to_string()),
    ];

    let allocator = RandomFairAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate();
    assert!(allocations.is_err());
  }

  #[test]
  fn test_presentation_allocator_no_projects() {
    let config = Config::default();

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
    ];

    let projects = vec![];

    let allocator = PresentationAllocator::new(config, judges, projects);

    let allocations = allocator.allocate().unwrap();
    for allocation in &allocations.allocations {
      assert_eq!(allocation.projects.len(), 0);
      assert_eq!(allocation.projects, allocator.projects);
    }
  }

  #[test]
  fn test_presentation_allocator() {
    let config = Config::default();

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
    ];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
    ];

    let allocator = PresentationAllocator::new(config, judges, projects);

    let allocations = allocator.allocate().unwrap();
    for allocation in &allocations.allocations {
      assert_eq!(allocation.projects.len(), allocator.projects.len());
      assert_eq!(allocation.projects, allocator.projects);
    }
  }
}
