use rust_xlsxwriter::{Format, Workbook, Worksheet, XlsxError};

use crate::{
  allocate::{Allocation, Allocations},
  time::Time,
};

const PROJECT_HEADER: &str = "Project";
const TIME_HEADER: &str = "Time";
const TABLE_HEADER: &str = "Table";
const NOTES_HEADER: &str = "Notes";
const RANK_HEADER: &str = "Rank";

const PROJECT_COL: u16 = 0;
const TIME_COL: u16 = 1;
const TABLE_COL: u16 = 2;
const NOTES_COL: u16 = 3;
const RANK_COL: u16 = 4;

const PROJECT_COL_WIDTH: f64 = 30.0;
const TIME_COL_WIDTH: f64 = 15.0;
const TABLE_COL_WIDTH: f64 = 10.0;
const NOTES_COL_WIDTH: f64 = 40.0;
const RANK_COL_WIDTH: f64 = 10.0;

/// Configuration for spreadsheet generation.
pub struct SpreadsheetConfig {
  /// Path to save the spreadsheet.
  pub output_path: String,
  /// Time allocated per project in minutes.
  pub judge_time: u32,
  /// Time that judging begins.
  pub start_time: Time,
}

impl SpreadsheetConfig {
  pub fn new(output_path: String, judge_time: u32, start_time: Time) -> Self {
    SpreadsheetConfig {
      output_path,
      judge_time,
      start_time,
    }
  }
}

pub struct Spreadsheet {
  /// Path to the spreadsheet.
  pub path: String,
  /// Config for the spreadsheet.
  pub config: SpreadsheetConfig,
}

impl Spreadsheet {
  /// Create a new Spreadsheet instance.
  /// `path` is the file path where the spreadsheet will be saved.
  pub fn new(path: String, config: SpreadsheetConfig) -> Self {
    Spreadsheet { path, config }
  }

  /// Create a spreadsheet from allocations where each judge gets their own sheet.
  /// Each sheet contains columns: Project, Time, Table, Notes, Rank
  pub fn from_allocations(allocations: &Allocations, config: &SpreadsheetConfig) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let header_format = Format::new().set_bold();

    for allocation in &allocations.allocations {
      let worksheet = workbook.add_worksheet();
      worksheet.set_name(&allocation.judge.name)?;

      Self::write_judge_sheet(worksheet, allocation, config, &header_format)?;
    }

    workbook.save(&config.output_path)?;
    Ok(())
  }

  /// Write a sheet for a judges allocation.
  /// `worksheet` is the worksheet to write to.
  fn write_judge_sheet(
    worksheet: &mut Worksheet,
    allocation: &Allocation,
    config: &SpreadsheetConfig,
    header_format: &Format,
  ) -> Result<(), XlsxError> {
    let headers = [PROJECT_HEADER, TIME_HEADER, TABLE_HEADER, NOTES_HEADER, RANK_HEADER];

    for (col, header) in headers.iter().enumerate() {
      worksheet.write_string_with_format(0, col as u16, *header, header_format)?;
    }

    worksheet.set_column_width(PROJECT_COL, PROJECT_COL_WIDTH)?; // Project
    worksheet.set_column_width(TIME_COL, TIME_COL_WIDTH)?; // Time
    worksheet.set_column_width(TABLE_COL, TABLE_COL_WIDTH)?; // Table
    worksheet.set_column_width(NOTES_COL, NOTES_COL_WIDTH)?; // Notes
    worksheet.set_column_width(RANK_COL, RANK_COL_WIDTH)?; // Rank
    let mut current_minutes = config.start_time.to_minutes();

    for (i, project) in allocation.projects.iter().enumerate() {
      let row = (i + 1) as u32;

      worksheet.write_string(row, PROJECT_COL, &project.name)?;

      let time = Time::from_minutes(current_minutes);
      worksheet.write_string(row, TIME_COL, &time.format())?; // Time
      worksheet.write_string(row, TABLE_COL, &project.id)?; // Table
      worksheet.write_string(row, NOTES_COL, "")?; // Notes
      worksheet.write_string(row, RANK_COL, "")?; // Rank

      current_minutes += config.judge_time;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::judge::Judge;
  use crate::project::Project;

  #[test]
  fn test_spreadsheet_creation() {
    let judge1 = Judge::new("1".to_string(), "Judge 1".to_string());
    let judge2 = Judge::new("2".to_string(), "Judge 2".to_string());

    let projects = vec![
      Project::new("1".to_string(), "Project 1".to_string()),
      Project::new("2".to_string(), "Project 2".to_string()),
      Project::new("3".to_string(), "Project 3".to_string()),
    ];

    let allocations = Allocations::new(vec![
      Allocation::new(judge1, projects.clone()),
      Allocation::new(judge2, projects),
    ]);

    let config = SpreadsheetConfig::new("test.xlsx".to_string(), 10, Time::new(9, 0).unwrap());

    let result = Spreadsheet::from_allocations(&allocations, &config);
    assert!(result.is_ok());

    let _ = std::fs::remove_file("test.xlsx");
  }
}
