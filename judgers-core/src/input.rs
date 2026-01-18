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
