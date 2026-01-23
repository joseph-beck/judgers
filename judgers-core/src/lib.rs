pub mod allocate;
pub mod error;
pub mod format;
pub mod input;
pub mod judge;
pub mod mode;
pub mod order;
pub mod project;
pub mod scoring;
pub mod spreadsheet;

pub trait Validate {
  fn validate(&self) -> Result<(), error::Error>;
}
