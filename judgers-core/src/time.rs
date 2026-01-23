use crate::error::Error;

/// Represents a time of day (hours and minutes).
#[derive(Clone, Debug, PartialEq)]
pub struct Time {
  /// Hour (0-23)
  pub hour: u8,
  /// Minute (0-59)
  pub minute: u8,
}

impl Time {
  /// Create a new Time.
  /// Errors if hour >= 24 or minute >= 60.
  pub fn new(hour: u8, minute: u8) -> Result<Self, Error> {
    if hour < 24 && minute < 60 {
      Ok(Time { hour, minute })
    } else {
      Err(Error::InvalidTime)
    }
  }

  /// Convert to total minutes since midnight.
  pub fn to_minutes(&self) -> u32 {
    (self.hour as u32) * 60 + (self.minute as u32)
  }

  /// Create a Time from total minutes since midnight.
  pub fn from_minutes(total: u32) -> Self {
    Time {
      hour: ((total / 60) % 24) as u8,
      minute: (total % 60) as u8,
    }
  }

  /// Parse from "HH:MM" format.
  pub fn parse(s: &str) -> Result<Self, Error> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
      return Err(Error::InvalidTime);
    }

    let hour: u8 = parts[0].parse().map_err(|_| Error::InvalidTime)?;
    let minute: u8 = parts[1].parse().map_err(|_| Error::InvalidTime)?;

    Time::new(hour, minute)
  }

  /// Format as "HH:MM".
  pub fn format(&self) -> String {
    format!("{:02}:{:02}", self.hour, self.minute)
  }
}

impl Default for Time {
  fn default() -> Self {
    Time { hour: 9, minute: 0 } // 09:00
  }
}
