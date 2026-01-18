use clap::{Arg, Command};
use judgers_core::{
  allocate::Allocator,
  config::{Config, Mode},
  error,
  input::Input,
};
use serde_json::from_str;
use std::fs;

use crate::style;

pub fn run() -> Result<(), error::Error> {
  let matches = command().get_matches();

  match matches.subcommand() {
    Some(("allocate", sub_matches)) => {
      let file = sub_matches.get_one::<String>("file").unwrap();
      let output = sub_matches.get_one::<String>("output");
      let mode = sub_matches.get_one::<String>("mode");
      let allocator = sub_matches.get_one::<String>("allocator");
      let judge_count = sub_matches.get_one::<String>("judge");
      let time_per_judge = sub_matches.get_one::<String>("time");

      let contents = fs::read_to_string(file).unwrap();
      let input = from_str::<Input>(&contents).unwrap();

      let mut config = Config::default();

      if let Some(count) = judge_count {
        config.judge_amount_min = count.parse::<u32>().unwrap_or(config.judge_amount_min);
      }

      if let Some(time) = time_per_judge {
        config.judge_time = time.parse::<u32>().unwrap_or(config.judge_time);
      }

      if let Some(m) = mode {
        config.mode = Mode::from_str(Some(m.clone())).unwrap_or(Mode::Json);
      }

      if let Some(out) = output {
        config.output_path = out.clone();
      }

      let alloc = <dyn Allocator>::from_str(
        allocator.map(|s| s.as_str()).unwrap_or("random"),
        config,
        input.judges,
        input.projects,
      );

      match alloc.allocate() {
        Ok(allocations) => {
          println!("allocations: {:?}", allocations);
          Ok(())
        }
        Err(e) => Err(e),
      }
    }
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

  let mode_arg = Arg::new("mode")
    .short('m')
    .long("mode")
    .help("choose the output mode: json or xlsx");

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
        .arg(mode_arg)
        .arg(judge_arg)
        .arg(time_arg),
    )
}
