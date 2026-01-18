#[derive(Debug, PartialEq)]
/// Various Error types that can occur when using the judgers-core crate.
pub enum Error {
  /// When there are not enough judges as per the configuration.
  ErrNotEnoughJudges {
    judge_count: usize,
    project_count: usize,
    judge_amount_min: u32,
  },
  /// When there are no judges.
  ErrNoJudges,
  /// When there are no projects.
  ErrNoProjects,
}
