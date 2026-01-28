use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{error, format::Format, judge::Judge, project::Project};

/// Allocator trait, must be implemented by all allocators.
pub trait Allocator {
  /// Allocate, returns a mapping of judges to projects.
  /// May return an error if allocation is not possible.
  fn allocate(&self) -> Result<Allocations, error::Error>;
}

impl dyn Allocator {
  /// Create a new allocator using a string identifier.
  /// Defaults to using the RandomFairAllocator.
  pub fn from_str(
    allocator: &str,
    config: AllocationConfig,
    judges: Vec<Judge>,
    projects: Vec<Project>,
  ) -> Box<dyn Allocator> {
    match allocator {
      "random" => Box::new(RandomFairAllocator::new(config, judges, projects)),
      "sequence" => Box::new(SequenceFairAllocator::new(config, judges, projects)),
      "presentation" => Box::new(PresentationAllocator::new(config, judges, projects)),
      _ => Box::new(RandomFairAllocator::new(config, judges, projects)),
    }
  }
}

/// Configuration for automatically generating judge allocations for projects with judges.
/// Requires that for a given format some options be populated.
#[derive(Clone)]
pub struct AllocationConfig {
  /// Minimum amount of times a project needs to be judged.
  /// Defaults to 3.
  pub judge_amount_min: u32,
  /// Amount of time each judge has to judge each project, in minutes.
  /// Defaults to 5.
  pub judge_time: u32,
  /// What format are we generating judging results for?
  /// Json or Spreadsheet (Xlsx)
  /// Defaults to using Json.
  pub format: Format,
  /// Where should the result be output to?
  /// Defaults to current working directory.
  pub output_path: Option<String>,
}

impl Default for AllocationConfig {
  fn default() -> Self {
    AllocationConfig {
      judge_amount_min: 3,
      judge_time: 5,
      format: Format::Json,
      output_path: None,
    }
  }
}

impl AllocationConfig {
  pub fn new(judge_amount_min: u32, judge_time: u32, format: Format, output_path: Option<String>) -> Self {
    AllocationConfig {
      judge_amount_min,
      judge_time,
      format,
      output_path,
    }
  }
}

impl std::fmt::Debug for AllocationConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Config")
      .field("judge_amount_min", &self.judge_amount_min)
      .field("judge_time", &self.judge_time)
      .field("format", &self.format)
      .field("output_path", &self.output_path)
      .finish()
  }
}

/// Allocation for a single judge and their assigned projects.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone)]
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
  config: AllocationConfig,
  /// All judges that are used for allocations.
  judges: Vec<Judge>,
  /// All projects that will be assigned to judges.
  projects: Vec<Project>,
}

impl RandomFairAllocator {
  pub fn new(config: AllocationConfig, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    RandomFairAllocator {
      config,
      judges,
      projects,
    }
  }
}

impl Allocator for RandomFairAllocator {
  /// Allocate projects to judges randomly.
  /// Each project will be assigned to at least the min judge count.
  /// May return an error if allocation is not possible.
  fn allocate(&self) -> Result<Allocations, error::Error> {
    if self.judges.is_empty() {
      return Err(error::Error::ErrNoJudges);
    }

    if self.projects.is_empty() {
      return Err(error::Error::ErrNoProjects);
    }

    let mut allocations: Vec<Allocation> = Vec::new();

    for judge in &self.judges {
      allocations.push(Allocation::new(judge.clone(), Vec::new()));
    }

    if self.config.judge_amount_min > self.judges.len() as u32 {
      return Err(error::Error::ErrNotEnoughJudges {
        judge_count: self.judges.len(),
        project_count: self.projects.len(),
        judge_amount_min: self.config.judge_amount_min,
      });
    }

    for project in &self.projects {
      let mut judges_allocated = 0;
      while judges_allocated < self.config.judge_amount_min {
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

/// Similar to the allocation of the random fair allocator,
/// but judges are assigned in sequence rather than randomly.
/// This means judges will judge in the order, x then y then z.
pub struct SequenceFairAllocator {
  /// General configuration for allocators.
  config: AllocationConfig,
  /// All judges that are used for allocations.
  judges: Vec<Judge>,
  /// All projects that will be assigned to judges.
  projects: Vec<Project>,
}

impl SequenceFairAllocator {
  pub fn new(config: AllocationConfig, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    SequenceFairAllocator {
      config,
      judges,
      projects,
    }
  }
}

impl Allocator for SequenceFairAllocator {
  /// Allocate projects to judges in sequence.
  /// Each project will be assigned to at least the min judge count.
  /// May return an error if allocation is not possible.
  fn allocate(&self) -> Result<Allocations, error::Error> {
    if self.judges.is_empty() {
      return Err(error::Error::ErrNoJudges);
    }

    if self.projects.is_empty() {
      return Err(error::Error::ErrNoProjects);
    }

    let mut allocations: Vec<Allocation> = Vec::new();

    for judge in &self.judges {
      allocations.push(Allocation::new(judge.clone(), Vec::new()));
    }

    if self.config.judge_amount_min > self.judges.len() as u32 {
      return Err(error::Error::ErrNotEnoughJudges {
        judge_count: self.judges.len(),
        project_count: self.projects.len(),
        judge_amount_min: self.config.judge_amount_min,
      });
    }

    let num_judges = self.judges.len();
    let num_projects = self.projects.len();

    let judges_per_project = self.config.judge_amount_min as usize;
    let projects_per_judge = ((num_projects * judges_per_project) as f64 / num_judges as f64).ceil() as usize;

    for (i, allocation) in allocations.iter_mut().enumerate() {
      // start offset is required to prevent judges from judging the same project at the same time.
      // for example j1 will judge p1 whilst j2 judges p2, etc.
      let start_offset = (i * num_projects) / num_judges;

      for j in 0..projects_per_judge {
        let project_idx = (start_offset + j) % num_projects;

        if j < num_projects {
          allocation.projects.push(self.projects[project_idx].clone());
        }
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
  _config: AllocationConfig,
  /// All judges that are used for allocations.
  judges: Vec<Judge>,
  /// All projects that will be assigned to judges.
  projects: Vec<Project>,
}

impl PresentationAllocator {
  pub fn new(_config: AllocationConfig, judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    PresentationAllocator {
      _config,
      judges,
      projects,
    }
  }
}

impl Allocator for PresentationAllocator {
  /// Allocate all projects to all judges.
  /// May return an error if allocation is not possible.
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

  use std::collections::HashMap;

  #[test]
  fn test_random_allocator_with_two() {
    let config = AllocationConfig {
      judge_amount_min: 2,
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
    let config = AllocationConfig {
      judge_amount_min: 3,
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
    let config = AllocationConfig {
      judge_amount_min: 3,
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
  fn test_random_allocator_no_projects() {
    let config = AllocationConfig {
      judge_amount_min: 3,
      ..Default::default()
    };

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
      Judge::new("3".to_string(), "Judge 3".to_string()),
    ];

    let projects = vec![];

    let allocator = RandomFairAllocator::new(config, judges, projects);

    let result = allocator.allocate();

    assert!(result.is_err());
  }

  #[test]
  fn test_random_allocator_no_judges() {
    let config = AllocationConfig {
      judge_amount_min: 3,
      ..Default::default()
    };

    let judges = vec![];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
    ];

    let allocator = RandomFairAllocator::new(config, judges, projects);

    let result = allocator.allocate();

    assert!(result.is_err());
  }

  #[test]
  fn test_sequence_allocator_with_two() {
    let config = AllocationConfig {
      judge_amount_min: 2,
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

    let allocator = SequenceFairAllocator::new(config, judges.clone(), projects.clone());
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
  fn test_sequence_allocator_three() {
    let config = AllocationConfig {
      judge_amount_min: 3,
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
    ];

    let allocator = SequenceFairAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate().unwrap();

    let mut project_counts: HashMap<String, usize> = HashMap::new();

    for allocation in &allocations.allocations {
      for project in &allocation.projects {
        *project_counts.entry(project.id.clone()).or_insert(0) += 1;
      }
    }

    for project in &projects {
      assert!(
        project_counts.get(&project.id).is_some_and(|&count| count >= 2),
        "Project {} was not allocated exactly twice",
        project.name
      );
    }
  }

  #[test]
  fn test_sequence_allocator_error_not_enough_judges() {
    let config = AllocationConfig {
      judge_amount_min: 3,
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

    let allocator = SequenceFairAllocator::new(config, judges.clone(), projects.clone());
    let allocations = allocator.allocate();
    assert!(allocations.is_err());
  }

  #[test]
  fn test_sequence_allocator_no_projects() {
    let config = AllocationConfig {
      judge_amount_min: 3,
      ..Default::default()
    };

    let judges = vec![
      Judge::new("1".to_string(), "Judge 1".to_string()),
      Judge::new("2".to_string(), "Judge 2".to_string()),
      Judge::new("3".to_string(), "Judge 3".to_string()),
    ];

    let projects = vec![];

    let allocator = SequenceFairAllocator::new(config, judges, projects);

    let result = allocator.allocate();

    assert!(result.is_err());
  }

  #[test]
  fn test_sequence_allocator_no_judges() {
    let config = AllocationConfig {
      judge_amount_min: 3,
      ..Default::default()
    };

    let judges = vec![];

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
    ];

    let allocator = SequenceFairAllocator::new(config, judges, projects);

    let result = allocator.allocate();

    assert!(result.is_err());
  }

  #[test]
  fn test_presentation_allocator_no_projects() {
    let config = AllocationConfig::default();

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
    let config = AllocationConfig::default();

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
