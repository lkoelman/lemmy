use crate::{
  extract_usernames, fetch_iframely_and_pictshare_data, generate_random_string, naive_from_unix,
  naive_now, remove_slurs, send_email, slur_check, slurs_vec_to_str,
};

use crate::settings::Settings;

use failure::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};

// Re-export request types & ops

pub mod schema;
pub mod handlers;
pub mod types;
pub mod ops;

// pub mod comment {
//   pub use ops::comment::*;
//   pub use types::comment::*;
// }
// pub mod community {
//   pub use ops::community::*;
//   pub use types::community::*;
// }
// pub mod post {
//   pub use ops::post::*;
//   pub use types::post::*;
// }
// pub mod site {
//   pub use ops::site::*;
//   pub use types::site::*;
// }
// pub mod user {
//   pub use ops::user::*;
//   pub use types::user::*;
// }


#[derive(Fail, Debug)]
#[fail(display = "{{\"error\":\"{}\"}}", message)]
pub struct APIError {
  pub message: String,
}

impl APIError {
  pub fn err(msg: &str) -> Self {
    APIError {
      message: msg.to_string(),
    }
  }
}

pub struct Oper<T> {
  data: T,
}

impl<T> Oper<T> {
  pub fn new(data: T) -> Oper<T> {
    Oper { data }
  }
}

// NOTE: there are two generic arguments:
//  - the input type T (= the form used)
//  = the result type Response (= the response type)
pub trait Perform<T> {
  type Response: Serialize + Send;
}


// TODO: define GraphQL/Juniper root queries and mutations here
// - dispatch them to api/ops/<type>.rs