use std::collections::HashMap;

use crate::{error, format::Format, mode::Mode, order::Order, project::Project};

/// Scorer trait, must be implemented by all scorers.
pub trait Scorer {
  /// Score, returns all projects and their scores.
  /// May return an error if scoring fails.
  fn score(&self) -> Result<Scores, error::Error>;
}

/// Configuration for automatically scoring projects based on judge decisions.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScorerConfig {
  pub format: Format,
  pub order: Order,
  pub mode: Mode,
}

impl ScorerConfig {
  pub fn new(format: Format, order: Order, mode: Mode) -> Self {
    ScorerConfig { format, order, mode }
  }
}

impl Default for ScorerConfig {
  fn default() -> Self {
    Self::new(Format::Json, Order::ScoreDesc, Mode::Average)
  }
}

pub struct Score {
  pub project_name: String,
  pub score: f64,
}

pub struct Scores {
  pub scores: Vec<Score>,
}

impl Scores {
  pub fn new(scores: Vec<Score>) -> Self {
    Scores { scores }
  }
}

impl Default for Scores {
  fn default() -> Self {
    Scores { scores: Vec::new() }
  }
}

pub struct ScoreTable {
  scores: HashMap<String, (f64, u32)>,
}

impl ScoreTable {
  pub fn new(scores: HashMap<String, (f64, u32)>) -> Self {
    ScoreTable { scores }
  }

  pub fn get(&self, project_name: &str) -> Option<&(f64, u32)> {
    self.scores.get(project_name)
  }

  pub fn insert(&mut self, project_name: String, score: f64) {
    self.scores.insert(project_name, (score, 1));
  }

  pub fn add(&mut self, project_name: String, score: f64) {
    let entry = self.scores.entry(project_name).or_insert((0.0, 0));
    entry.0 += score;
    entry.1 += 1;
  }

  pub fn get_score(&self, project_name: &str) -> Option<f64> {
    self.scores.get(project_name).map(|(score, _)| *score)
  }

  pub fn get_average_score(&self, project_name: &str) -> Option<f64> {
    self.scores.get(project_name).map(
      |(score, count)| {
        if *count == 0 {
          0.0
        } else {
          *score / (*count as f64)
        }
      },
    )
  }

  pub fn get_count(&self, project_name: &str) -> Option<u32> {
    self.scores.get(project_name).map(|(_, count)| *count)
  }

  pub fn len(&self) -> usize {
    self.scores.len()
  }

  pub fn is_empty(&self) -> bool {
    self.scores.is_empty()
  }

  pub fn to_scores(&self, projects: Vec<Project>, config: ScorerConfig) -> Scores {
    let mut scores_vec = Vec::new();

    for project in projects {
      if config.mode == Mode::Average {
        if let Some(avg_score) = self.get_average_score(&project.name) {
          scores_vec.push(Score {
            project_name: project.name.clone(),
            score: avg_score,
          });
        }
      } else {
        if let Some(total_score) = self.get_score(&project.name) {
          scores_vec.push(Score {
            project_name: project.name.clone(),
            score: total_score,
          });
        }
      }
    }

    Scores::new(scores_vec)
  }
}

impl Default for ScoreTable {
  fn default() -> Self {
    ScoreTable { scores: HashMap::new() }
  }
}

pub struct StackRankDecision {
  pub judge_id: String,
  pub ranks: Vec<(String, u32)>,
}

pub struct StackRankScorer {
  config: ScorerConfig,
  judge_stack_decisions: Vec<StackRankDecision>,
  projects: Vec<Project>,
  rank_weights: HashMap<u32, f64>,
}

impl StackRankScorer {
  pub fn new(
    config: ScorerConfig,
    judge_stack_decisions: Vec<StackRankDecision>,
    projects: Vec<Project>,
    rank_weights: HashMap<u32, f64>,
  ) -> Self {
    StackRankScorer {
      config,
      judge_stack_decisions,
      projects,
      rank_weights,
    }
  }
}

impl Scorer for StackRankScorer {
  fn score(&self) -> Result<Scores, error::Error> {
    let mut results = ScoreTable::default();

    for decision in &self.judge_stack_decisions {
      for (project_name, rank) in &decision.ranks {
        if let Some(weight) = self.rank_weights.get(rank) {
          results.add(project_name.clone(), *weight);
        }
      }
    }

    let scores = results.to_scores(self.projects.clone(), self.config.clone());

    match self.config.order {
      Order::ScoreAsc => {
        let mut sorted_scores = scores.scores;
        sorted_scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        Ok(Scores::new(sorted_scores))
      }
      Order::ScoreDesc => {
        let mut sorted_scores = scores.scores;
        sorted_scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Ok(Scores::new(sorted_scores))
      }
      Order::ProjectNameAsc => {
        let mut sorted_scores = scores.scores;
        sorted_scores.sort_by(|a, b| a.project_name.cmp(&b.project_name));
        Ok(Scores::new(sorted_scores))
      }
      Order::ProjectNameDesc => {
        let mut sorted_scores = scores.scores;
        sorted_scores.sort_by(|a, b| b.project_name.cmp(&a.project_name));
        Ok(Scores::new(sorted_scores))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stack_rank_scorer_score() {
    let config = ScorerConfig::default();

    let judge_stack_decisions = vec![
      StackRankDecision {
        judge_id: "1".to_string(),
        ranks: vec![
          ("project a".to_string(), 1),
          ("project b".to_string(), 2),
          ("project c".to_string(), 3),
        ],
      },
      StackRankDecision {
        judge_id: "2".to_string(),
        ranks: vec![
          ("project b".to_string(), 1),
          ("project c".to_string(), 2),
          ("project a".to_string(), 3),
        ],
      },
    ];

    let projects = vec![
      Project {
        id: "a".to_string(),
        name: "project a".to_string(),
      },
      Project {
        id: "b".to_string(),
        name: "project b".to_string(),
      },
      Project {
        id: "c".to_string(),
        name: "project c".to_string(),
      },
    ];

    let rank_weights = HashMap::from([(1, 3.0), (2, 2.0), (3, 1.0)]);

    let scorer = StackRankScorer::new(config, judge_stack_decisions, projects, rank_weights);

    let scores = scorer.score().unwrap();

    assert_eq!(scores.scores.len(), 3);
    for score in scores.scores {
      match score.project_name.as_str() {
        "project a" => assert_eq!(score.score, 2.0),
        "project b" => assert_eq!(score.score, 2.5),
        "project c" => assert_eq!(score.score, 1.5),
        _ => assert!(false),
      }
    }
  }
}
