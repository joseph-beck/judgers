use clap::Command;
use judgers_core::error;

use crate::style;

pub fn run() -> Result<(), error::Error> {
  let matches = command().get_matches();

  match matches.subcommand() {
    Some(("judgers", _)) => {
      println!("123");
      Ok(())
    }
    Some(("allocate", _)) => {
      println!("allocate");
      Ok(())
    }
    _ => unreachable!(),
  }
}

fn command() -> Command {
  Command::new("judgers")
    .about("judgers!")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .styles(style::style())
    .subcommand(Command::new("allocate").about("allocate judges to projects"))
}
