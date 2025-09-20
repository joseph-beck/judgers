// Mode for which judging allocations can be generated for.
#[derive(PartialEq, Eq)]
pub enum Mode {
  Json,
  Xlsx,
}

/// Configuration for automatically generating judge allocations for projects with judges.
/// Requires that for a given mode some options be populated.
/// For Xlsx mode, spreadsheet_path must be populated.
pub struct Config {
  /// Amount of times a project needs to be judged.
  /// Defaults to 3.
  pub judge_amount: u32,
  /// Should we enforce that all judges judge the same amount of projects?
  /// Defaults to false.
  pub judge_equal_amount: bool,
  /// Amount of time each judge has to judge each project, in minutes.
  /// Defaults to 5.
  pub judge_time: u32,
  /// What mode are we generating judging results for?
  /// Json or Spreadsheet (Xlsx)
  /// Defaults to using Json.
  pub mode: Mode,
  /// Where is the spreadsheet located?
  /// Only used if mode is Some(Mode::Xlsx)
  /// Defaults to None.
  pub spreadsheet_path: Option<String>,
  /// Where are the judges located?
  /// Defaults to None.
  pub judges_path: Option<String>,
  /// Where are the projects located?
  /// Defaults to None.
  pub projects_path: Option<String>,
  /// Who are the judges?
  /// Defaults to None.
  pub judges: Option<Vec<String>>,
  /// What are the projects?
  /// Defaults to None.
  pub projects: Option<Vec<String>>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      judge_amount: 3,
      judge_equal_amount: false,
      judge_time: 5,
      mode: Mode::Json,
      spreadsheet_path: None,
      judges_path: None,
      projects_path: None,
      judges: None,
      projects: None,
    }
  }
}
