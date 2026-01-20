mod cli;
mod handlers;
mod style;

fn main() {
  match cli::run() {
    Ok(_) => {}
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(1);
    }
  }
}
