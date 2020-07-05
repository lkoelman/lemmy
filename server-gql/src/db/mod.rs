/**
 * Module containing generic traits for all DB types.
 */
use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use dgraph_tonic as dgraph;
use dgraph_tonic::{Query, Mutation}; // ,DgraphError as Error

// re-export common functionality
// FIXME: possibly replace by: mod entity; pub use entity::category;
#[path = "entity/category.rs"]
pub mod category;
#[path = "entity/comment.rs"]
pub mod comment;
#[path = "entity/community.rs"]
pub mod community;
#[path = "entity/moderator.rs"]
pub mod moderator;
#[path = "entity/password_reset_request.rs"]
pub mod password_reset_request;
#[path = "entity/post.rs"]
pub mod post;
#[path = "entity/private_message.rs"]
pub mod private_message;
#[path = "entity/site.rs"]
pub mod site;
#[path = "entity/user.rs"]
pub mod user;
#[path = "entity/user_mention.rs"]
pub mod user_mention;

// Propagate any error
type Result<T> = std::result::Result<T, failure::Error>;


pub trait MaybeOptional<T> {
  fn get_optional(self) -> Option<T>;
}

impl<T> MaybeOptional<T> for T {
  fn get_optional(self) -> Option<T> {
    Some(self)
  }
}

impl<T> MaybeOptional<T> for Option<T> {
  fn get_optional(self) -> Option<T> {
    self
  }
}


#[derive(EnumString, ToString, Debug, Serialize, Deserialize)]
pub enum SortType {
  Hot,
  New,
  TopDay,
  TopWeek,
  TopMonth,
  TopYear,
  TopAll,
}

#[derive(EnumString, ToString, Debug, Serialize, Deserialize)]
pub enum ListingType {
  All,
  Subscribed,
  Community,
}

#[derive(EnumString, ToString, Debug, Serialize, Deserialize)]
pub enum SearchType {
  All,
  Comments,
  Posts,
  Communities,
  Users,
  Url,
}

pub fn fuzzy_search(q: &str) -> String {
  let replaced = q.replace(" ", "%");
  format!("%{}%", replaced)
}

pub fn limit_and_offset(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
  let page = page.unwrap_or(1);
  let limit = limit.unwrap_or(10);
  let offset = limit * (page - 1);
  (limit, offset)
}

#[cfg(test)]
mod tests {
  use super::fuzzy_search;
  #[test]
  fn test_fuzzy_search() {
    let test = "This is a fuzzy search";
    assert_eq!(fuzzy_search(test), "%This%is%a%fuzzy%search%".to_string());
  }
}
