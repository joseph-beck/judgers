pub mod allocate;
pub mod config;
pub mod error;
pub mod input;
pub mod judge;
pub mod project;
pub mod spreadsheet;

pub trait Validate {
  fn validate(&self) -> Result<(), error::Error>;
}
