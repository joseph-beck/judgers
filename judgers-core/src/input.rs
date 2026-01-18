use std::collections::HashSet;

use crate::{error::Error, judge::Judge, project::Project, Validate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
  pub judges: Vec<Judge>,
  pub projects: Vec<Project>,
}

impl Input {
  pub fn new(judges: Vec<Judge>, projects: Vec<Project>) -> Self {
    Input { judges, projects }
  }
}

impl Validate for Input {
  fn validate(&self) -> Result<(), Error> {
    if self.judges.is_empty() {
      return Err(Error::ErrNoJudges);
    }

    if self.projects.is_empty() {
      return Err(Error::ErrNoProjects);
    }

    for judge in &self.judges {
      judge.validate()?;
    }

    let judge_ids: Vec<&String> = self.judges.iter().map(|j| &j.id).collect();
    let unique_judge_ids: HashSet<&String> = judge_ids.iter().cloned().collect();
    if judge_ids.len() != unique_judge_ids.len() {
      return Err(Error::ErrDuplicateJudgeIds);
    }

    for project in &self.projects {
      project.validate()?;
    }

    let project_ids: Vec<&String> = self.projects.iter().map(|p| &p.id).collect();
    let unique_project_ids: HashSet<&String> = project_ids.iter().cloned().collect();
    if project_ids.len() != unique_project_ids.len() {
      return Err(Error::ErrDuplicateProjectIds);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_input_validation_valid() {
    let valid_judge = Judge::new("j1".to_string(), "Judge One".to_string());
    let valid_project = Project::new("p1".to_string(), "Project One".to_string());

    let input = Input::new(vec![valid_judge.clone()], vec![valid_project.clone()]);

    assert!(input.validate().is_ok());
  }

  #[test]
  fn test_input_validation_no_judges() {
    let valid_project = Project::new("p1".to_string(), "Project One".to_string());

    let input = Input::new(vec![], vec![valid_project.clone()]);

    assert!(input.validate().is_err());
  }

  #[test]
  fn test_input_validation_no_projects() {
    let valid_judge = Judge::new("j1".to_string(), "Judge One".to_string());

    let input = Input::new(vec![valid_judge.clone()], vec![]);

    assert!(input.validate().is_err());
  }

  #[test]
  fn test_input_validation_duplicate_judge_ids() {
    let valid_judge = Judge::new("j1".to_string(), "Judge One".to_string());

    let input = Input::new(vec![valid_judge.clone(), valid_judge.clone()], vec![]);

    assert!(input.validate().is_err());
  }

  #[test]
  fn test_input_validation_duplicate_project_ids() {
    let valid_project = Project::new("p1".to_string(), "Project One".to_string());

    let input = Input::new(vec![], vec![valid_project.clone(), valid_project.clone()]);

    assert!(input.validate().is_err());
  }
}
