use std::fmt::{Result as FmtResult, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok = 200, 
  BadRequest = 400, 
  NotFound = 404,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok => "Ok",
      Self::BadRequest => "BadRequest",
      Self::NotFound => "NotFound",
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", *self as u16)
  }
}