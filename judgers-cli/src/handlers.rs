use std::fs;

use judgers_core::{
  allocate::{AllocationConfig, Allocator},
  error::Error,
  format::Format,
  input::Input,
};
use serde_json::from_str;

use crate::cli::AllocateArgs;

pub fn handle_allocate(args: AllocateArgs) -> Result<(), Error> {
  let contents = fs::read_to_string(args.file).unwrap();
  let input = from_str::<Input>(&contents).unwrap();

  let mut config = AllocationConfig::default();

  if let Some(count) = args.judge_count {
    config.judge_amount_min = count
  }

  if let Some(time) = args.time_per_judge {
    config.judge_time = time;
  }

  if let Some(f) = args.format {
    config.format = Format::from_str(Some(f.clone())).unwrap_or(Format::Json);
  }

  if let Some(output) = args.output {
    config.output_path = Some(output);
  }

  let format = config.format.clone();

  let output = config.output_path.clone();

  let allocator = <dyn Allocator>::from_str(args.allocator.as_str(), config, input.judges, input.projects);

  let allocation_result = allocator.allocate();

  match allocation_result {
    Ok(allocation) => {
      if format == Format::Json {
        let json_output = serde_json::to_string_pretty(&allocation);

        if output.is_some() {
          fs::write(output.unwrap(), json_output.unwrap()).unwrap();
        } else {
          println!("{}", json_output.unwrap());
        }
      }

      Ok(())
    }
    Err(e) => Err(e),
  }
}

pub fn handle_score() -> Result<(), Error> {
  Ok(())
}
