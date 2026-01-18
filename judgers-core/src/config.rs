// Mode for which judging allocations can be generated for.
#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
  Json,
  Xlsx,
}

impl Mode {
  pub fn from_str(mode: Option<String>) -> Option<Mode> {
    match mode.as_deref() {
      Some("json") => Some(Mode::Json),
      Some("xlsx") => Some(Mode::Xlsx),
      _ => Some(Mode::Json),
    }
  }
}

/// Configuration for automatically generating judge allocations for projects with judges.
/// Requires that for a given mode some options be populated.
/// For Xlsx mode, spreadsheet_path must be populated.
pub struct Config {
  /// Minimum amount of times a project needs to be judged.
  /// Defaults to 3.
  pub judge_amount_min: u32,
  /// Amount of time each judge has to judge each project, in minutes.
  /// Defaults to 5.
  pub judge_time: u32,
  /// What mode are we generating judging results for?
  /// Json or Spreadsheet (Xlsx)
  /// Defaults to using Json.
  pub mode: Mode,
  /// Where should the result be output to?
  /// Defaults to current working directory.
  pub output_path: String,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      judge_amount_min: 3,
      judge_time: 5,
      mode: Mode::Json,
      output_path: ".".to_string(),
    }
  }
}

impl Config {
  pub fn new(judge_amount_min: u32, judge_time: u32, mode: Mode, output_path: String) -> Self {
    Config {
      judge_amount_min,
      judge_time,
      mode,
      output_path,
    }
  }
}

impl std::fmt::Debug for Config {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Config")
      .field("judge_amount_min", &self.judge_amount_min)
      .field("judge_time", &self.judge_time)
      .field("mode", &self.mode)
      .field("output_path", &self.output_path)
      .finish()
  }
}
