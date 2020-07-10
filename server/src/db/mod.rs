/**
 * Module containing generic traits for all DB types.
 */
use crate::settings::Settings;
use async_trait::async_trait;
use failure::Error;
use serde::{
  Deserialize, Serialize, Serializer, //, Deserializer,
  de::DeserializeOwned, ser::SerializeStruct
};
use serde_aux::prelude::deserialize_number_from_string;
use dgraph_tonic as dgraph;
use dgraph_tonic::{Client, Query, Mutation, Mutate};
use getset::{Setters, Getters, CopyGetters};

pub mod common;
pub use common::traits::*;

// NOTE: Alternative (subdir without module) is to declare .rs files
//       as "pub mod file" and annotate with path macro attribute
mod entity;
pub use entity::{
  category, comment, community, moderator, password_reset_request,
  post, private_message, site, user, user_mention,
};

mod query;
pub use query::{
  comment_view, community_view, moderator_views, post_view,
  private_message_view, site_view, user_mention_view, user_view,
};

// Propagate any error
// type Result<T> = std::result::Result<T, failure::Error>;


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
