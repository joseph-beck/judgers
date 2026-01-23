// Format for which judging allocations can be generated for.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Format {
  Json,
  Xlsx,
}

impl Format {
  pub fn from_str(mode: Option<String>) -> Option<Format> {
    match mode.as_deref() {
      Some("json") => Some(Format::Json),
      Some("xlsx") => Some(Format::Xlsx),
      _ => Some(Format::Json),
    }
  }
}
