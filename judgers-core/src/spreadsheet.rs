use std::collections::HashMap;

use rust_xlsxwriter::{Format, Workbook, Worksheet, XlsxError};
use serde::{Deserialize, Serialize};

use crate::{
  allocate::{Allocation, Allocations},
  project::Project,
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

const CONFIG_SHEET_NAME: &str = "Score Configuration";
const CONFIG_POINTS_COL_HEADER: &str = "Points";
const CONFIG_RANK_COL_HEADER: &str = "Rank";

const RESULTS_SHEET_NAME: &str = "Results";
const RESULTS_PROJECT_COL_HEADER: &str = "Project";
const RESULTS_TOTAL_POINTS_COL_HEADER: &str = "Total Points";
const RESULTS_AVG_RANK_COL_HEADER: &str = "Average Rank";
const RESULTS_AVG_POINTS_COL_HEADER: &str = "Average Points";
const RESULTS_JUDGE_RANK_COL_HEADER: &str = " Rank";
const RESULTS_JUDGE_POINTS_COL_HEADER: &str = " Points";

/// Configuration for spreadsheet generation.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpreadsheetConfig {
  /// Path to save the spreadsheet.
  pub output_path: String,
  /// Time allocated per project in minutes.
  pub judge_time: u32,
  /// Time that judging begins.
  pub start_time: Time,
  /// Rank weights mapping rank position to points.
  /// e.g., {1: 10.0, 2: 8.0, 3: 6.0} means 1st place = 10 points, etc.
  pub rank_weights: HashMap<u32, f64>,
}

impl SpreadsheetConfig {
  pub fn new(output_path: String, judge_time: u32, start_time: Time, rank_weights: HashMap<u32, f64>) -> Self {
    SpreadsheetConfig {
      output_path,
      judge_time,
      start_time,
      rank_weights,
    }
  }

  /// Create config with default rank weights (1st=10, 2nd=8, 3rd=6, 4th=4, 5th=2).
  pub fn with_default_weights(output_path: String, judge_time: u32, start_time: Time) -> Self {
    let mut rank_weights = HashMap::new();
    rank_weights.insert(1, 10.0);
    rank_weights.insert(2, 8.0);
    rank_weights.insert(3, 6.0);
    rank_weights.insert(4, 4.0);
    rank_weights.insert(5, 2.0);
    Self::new(output_path, judge_time, start_time, rank_weights)
  }
}

impl Default for SpreadsheetConfig {
  fn default() -> Self {
    Self::with_default_weights("judging-schedule.xlsx".to_string(), 10, Time::new(9, 0).unwrap())
  }
}

pub struct Spreadsheet {
  /// Config for the spreadsheet.
  pub config: SpreadsheetConfig,
}

impl Spreadsheet {
  /// Create a new Spreadsheet instance.
  /// `path` is the file path where the spreadsheet will be saved.
  pub fn new(config: SpreadsheetConfig) -> Self {
    Spreadsheet { config }
  }

  /// Create a spreadsheet from allocations where each judge gets their own sheet.
  /// Each sheet contains columns: Project, Time, Table, Notes, Rank
  /// Also creates a Score Configuration sheet and a Results sheet.
  pub fn from_allocations(&self, allocations: &Allocations) -> Result<(), XlsxError> {
    let config = &self.config;
    let mut workbook = Workbook::new();

    let header_format = Format::new().set_bold();

    let all_projects = Self::collect_unique_projects(allocations);
    let judge_names: Vec<String> = allocations.allocations.iter().map(|a| a.judge.name.clone()).collect();

    // Create a sheet for each judge
    for allocation in &allocations.allocations {
      let worksheet = workbook.add_worksheet();
      worksheet.set_name(&allocation.judge.name)?;

      Self::write_judge_sheet(worksheet, allocation, config, &header_format)?;
    }

    // Create Score Configuration sheet
    let score_config_sheet = workbook.add_worksheet();
    score_config_sheet.set_name(CONFIG_SHEET_NAME)?;
    Self::write_score_config_sheet(score_config_sheet, &config.rank_weights, &header_format)?;

    // Create Results sheet
    let results_sheet = workbook.add_worksheet();
    results_sheet.set_name(RESULTS_SHEET_NAME)?;
    Self::write_results_sheet(
      results_sheet,
      &all_projects,
      &judge_names,
      &config.rank_weights,
      &header_format,
    )?;

    workbook.save(&config.output_path)?;

    Ok(())
  }

  /// Collect all unique projects from allocations.
  fn collect_unique_projects(allocations: &Allocations) -> Vec<Project> {
    let mut seen = std::collections::HashSet::new();
    let mut projects = Vec::new();

    for allocation in &allocations.allocations {
      for project in &allocation.projects {
        if seen.insert(project.id.clone()) {
          projects.push(project.clone());
        }
      }
    }

    projects
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

      let time = Time::from_minutes(current_minutes);

      worksheet.write_string(row, PROJECT_COL, &project.name)?; // Project
      worksheet.write_string(row, TIME_COL, &time.format())?; // Time
      worksheet.write_string(row, TABLE_COL, &project.id)?; // Table
      worksheet.write_string(row, NOTES_COL, "")?; // Notes
      worksheet.write_string(row, RANK_COL, "")?; // Rank

      current_minutes += config.judge_time;
    }

    Ok(())
  }

  /// Write the Score Configuration sheet with rank-to-points mapping.
  fn write_score_config_sheet(
    worksheet: &mut Worksheet,
    rank_weights: &HashMap<u32, f64>,
    header_format: &Format,
  ) -> Result<(), XlsxError> {
    worksheet.write_string_with_format(0, 0, CONFIG_RANK_COL_HEADER, header_format)?;
    worksheet.write_string_with_format(0, 1, CONFIG_POINTS_COL_HEADER, header_format)?;

    worksheet.set_column_width(0, 10.0)?;
    worksheet.set_column_width(1, 10.0)?;

    // Sort ranks for consistent ordering
    let mut ranks: Vec<_> = rank_weights.iter().collect();
    ranks.sort_by_key(|(rank, _)| *rank);

    for (row_idx, (rank, points)) in ranks.iter().enumerate() {
      let row = (row_idx + 1) as u32;
      worksheet.write_number(row, 0, **rank as f64)?;
      worksheet.write_number(row, 1, **points)?;
    }

    Ok(())
  }

  /// Write the Results sheet with projects, ranks from judge sheets, and points calculation.
  fn write_results_sheet(
    worksheet: &mut Worksheet,
    projects: &[Project],
    judge_names: &[String],
    rank_weights: &HashMap<u32, f64>,
    header_format: &Format,
  ) -> Result<(), XlsxError> {
    worksheet.write_string_with_format(0, 0, RESULTS_PROJECT_COL_HEADER, header_format)?;
    worksheet.set_column_width(0, 30.0)?;

    // Write judge name headers (Rank and Points for each judge)
    let mut col: u16 = 1;
    for judge_name in judge_names.iter() {
      // Rank column
      let rank_header = format!("{} {}", judge_name, RESULTS_JUDGE_RANK_COL_HEADER);
      worksheet.write_string_with_format(0, col, &rank_header, header_format)?;
      worksheet.set_column_width(col, 12.0)?;
      col += 1;

      // Points column
      let points_header = format!("{} {}", judge_name, RESULTS_JUDGE_POINTS_COL_HEADER);
      worksheet.write_string_with_format(0, col, &points_header, header_format)?;
      worksheet.set_column_width(col, 12.0)?;
      col += 1;
    }

    // Summary columns
    let total_points_col = col;
    worksheet.write_string_with_format(0, total_points_col, RESULTS_TOTAL_POINTS_COL_HEADER, header_format)?;
    worksheet.set_column_width(total_points_col, 12.0)?;
    col += 1;

    let avg_rank_col = col;
    worksheet.write_string_with_format(0, avg_rank_col, RESULTS_AVG_RANK_COL_HEADER, header_format)?;
    worksheet.set_column_width(avg_rank_col, 12.0)?;
    col += 1;

    let avg_points_col = col;
    worksheet.write_string_with_format(0, avg_points_col, RESULTS_AVG_POINTS_COL_HEADER, header_format)?;
    worksheet.set_column_width(avg_points_col, 14.0)?;

    let num_ranks = rank_weights.len();

    for (row, project) in projects.iter().enumerate() {
      let row = (row + 1) as u32;

      worksheet.write_string(row, 0, &project.name)?;

      let mut rank_cols: Vec<u16> = Vec::new();
      let mut points_cols: Vec<u16> = Vec::new();

      let mut current_col: u16 = 1;
      for judge_name in judge_names.iter() {
        // Rank column - VLOOKUP to find this project's rank in judge's sheet
        // =IFERROR(VLOOKUP(A2,'Judge 1'!A:E,5,FALSE),"")
        let rank_formula = format!("=IFERROR(VLOOKUP(A{},'{}'!A:E,5,FALSE),\"\")", row + 1, judge_name);
        worksheet.write_formula(row, current_col, rank_formula.as_str())?;
        rank_cols.push(current_col);
        current_col += 1;

        // Points column - VLOOKUP to convert rank to points using Score Configuration
        // =IFERROR(VLOOKUP(B2,'Score Configuration'!A:B,2,FALSE),"")
        let rank_cell = format!("{}{}", Self::column_letter(current_col - 1), row + 1);
        let points_formula = format!(
          "=IFERROR(VLOOKUP({rank_cell},'Score Configuration'!A$2:B${},2,FALSE),\"\")",
          num_ranks + 1
        );
        worksheet.write_formula(row, current_col, points_formula.as_str())?;
        points_cols.push(current_col);
        current_col += 1;
      }

      // Total Points formula - sum of all points columns
      let points_refs: Vec<String> = points_cols
        .iter()
        .map(|c| format!("{}{}", Self::column_letter(*c), row + 1))
        .collect();
      let total_points_formula = format!("=IFERROR(SUM({}),\"\")", points_refs.join(","));
      worksheet.write_formula(row, total_points_col, total_points_formula.as_str())?;

      // Average Rank formula - average of all rank columns
      let rank_refs: Vec<String> = rank_cols
        .iter()
        .map(|c| format!("{}{}", Self::column_letter(*c), row + 1))
        .collect();
      let avg_rank_formula = format!("=IFERROR(AVERAGE({}),\"\")", rank_refs.join(","));
      worksheet.write_formula(row, avg_rank_col, avg_rank_formula.as_str())?;

      // Average Points formula - average of all points columns
      let avg_points_formula = format!("=IFERROR(AVERAGE({}),\"\")", points_refs.join(","));
      worksheet.write_formula(row, avg_points_col, avg_points_formula.as_str())?;
    }

    Ok(())
  }

  /// Convert a 0-based column index to Excel column letter (0=A, 1=B, etc.).
  fn column_letter(col: u16) -> String {
    let mut result = String::new();
    let mut n = col as u32 + 1;

    while n > 0 {
      n -= 1;
      result.insert(0, (b'A' + (n % 26) as u8) as char);
      n /= 26;
    }

    result
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

    let config = SpreadsheetConfig::with_default_weights("test.xlsx".to_string(), 10, Time::new(9, 0).unwrap());

    let spreadsheet = Spreadsheet::new(config);
    let result = spreadsheet.from_allocations(&allocations);
    assert!(result.is_ok());

    let _ = std::fs::remove_file("test.xlsx");
  }
}
