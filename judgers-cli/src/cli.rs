use clap::{Arg, Command};
use judgers_core::error;

use crate::{
  handlers::{handle_allocate, handle_score},
  style,
};

pub struct AllocateArgs {
  pub file: String,
  pub output: Option<String>,
  pub format: Option<String>,
  pub allocator: String,
  pub judge_count: Option<u32>,
  pub time_per_judge: Option<u32>,
}

impl AllocateArgs {
  fn new(
    file: String,
    output: Option<String>,
    format: Option<String>,
    allocator: String,
    judge_count: Option<u32>,
    time_per_judge: Option<u32>,
  ) -> Self {
    AllocateArgs {
      file,
      output,
      format,
      allocator,
      judge_count,
      time_per_judge,
    }
  }
}

pub fn run() -> Result<(), error::Error> {
  let matches = command().get_matches();

  match matches.subcommand() {
    Some(("allocate", a)) => {
      let args = AllocateArgs::new(
        a.get_one::<String>("file").unwrap().to_string(),
        a.get_one::<String>("output").cloned(),
        a.get_one::<String>("format").cloned(),
        a.get_one::<String>("allocator").unwrap().to_string(),
        a.get_one::<String>("judge").map(|s| s.parse::<u32>().unwrap()),
        a.get_one::<String>("time").map(|s| s.parse::<u32>().unwrap()),
      );

      handle_allocate(args)
    }
    Some(("score", _)) => handle_score(),
    _ => unreachable!(),
  }
}

fn command() -> Command {
  let file_arg = Arg::new("file")
    .help("input file path containing judges and projects")
    .required(true)
    .index(1);

  let output_arg = Arg::new("output")
    .short('o')
    .long("output")
    .help("output file path to write judging allocations to");

  let format_arg = Arg::new("format")
    .short('f')
    .long("format")
    .help("choose the output format: json or xlsx");

  let allocator_arg = Arg::new("allocator")
    .short('a')
    .long("allocator")
    .help("choose the allocator you wish to use: random or sequence")
    .default_value("random");

  let judge_arg = Arg::new("judge")
    .short('j')
    .long("judge")
    .help("minimum number of judges per project");

  let time_arg = Arg::new("time")
    .short('t')
    .long("time")
    .help("time each judge has to judge each project, in minutes");

  Command::new("judgers")
    .about("judgers!")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .styles(style::style())
    .subcommand(
      Command::new("allocate")
        .about("allocate judges to projects")
        .arg(file_arg)
        .arg(allocator_arg)
        .arg(output_arg)
        .arg(format_arg)
        .arg(judge_arg)
        .arg(time_arg),
    )
    .subcommand(Command::new("score").about("score projects based on judge results"))
}
