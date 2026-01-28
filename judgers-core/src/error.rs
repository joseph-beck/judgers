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
  /// When the judge ID is invalid.
  ErrInvalidJudgeId(String),
  /// When there are duplicate judge IDs.
  ErrDuplicateJudgeIds,
  /// When the judge name is invalid.
  ErrInvalidJudgeName(String),
  /// When the project ID is invalid.
  ErrInvalidProjectId(String),
  /// When there are duplicate project IDs.
  ErrDuplicateProjectIds,
  /// When the project name is invalid.
  ErrInvalidProjectName(String),
  /// When no rank weights are provided for stack ranking.
  ErrNoRankWeights,
  /// When the time provided is invalid.
  /// Hour must be less than 24 and minute must be less than 60.
  ErrInvalidTime,
  /// When failed to create the spreadsheet.
  /// Includes the underlying error message.
  ErrFailedToCreateSpreadsheet(String),
}
