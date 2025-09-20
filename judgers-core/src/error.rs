#[derive(Debug, PartialEq)]
/// Various Error types that can occur when using the judgers-core crate.
pub enum Error {
  /// When there are no enough judges as per the configuration.
  NotEnoughJudges {
    judges: u32,
    projects: u32,
    judge_amount: u32,
  },
}
