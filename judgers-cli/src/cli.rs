use clap::{Arg, Command};
use judgers_core::{error, input::Input};
use serde_json::from_str;
use std::fs;

use crate::style;

pub fn run() -> Result<(), error::Error> {
  let matches = command().get_matches();

  match matches.subcommand() {
    Some(("allocate", sub_matches)) => {
      let file = sub_matches.get_one::<String>("file").unwrap();
      let output = sub_matches.get_one::<String>("output");
      let allocator = sub_matches.get_one::<String>("allocator");

      let contents = fs::read_to_string(file).unwrap();

      let json = from_str::<Input>(&contents).unwrap();

      println!("input: {:?}", json);
      println!("allocator: {:?}", allocator);
      println!("output: {:?}", output);

      Ok(())
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

  let allocator_arg = Arg::new("allocator")
    .short('a')
    .long("allocator")
    .help("choose the allocator you wish to use: random or sequence");

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
        .arg(output_arg),
    )
}
