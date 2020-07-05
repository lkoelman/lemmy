use serde::{Serialize, Deserialize};
use crate::db::category::{Category};
use super::comment::*;
use super::post::*;
use super::user::*;
use super::community::*;

#[derive(Serialize, Deserialize)]
pub struct ListCategories {}

#[derive(Serialize, Deserialize)]
pub struct ListCategoriesResponse {
  categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
pub struct Search {
  q: String,
  type_: String,
  community_id: Option<i32>,
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
  type_: String,
  comments: Vec<CommentView>,
  posts: Vec<PostView>,
  communities: Vec<CommunityView>,
  users: Vec<UserView>,
}

#[derive(Serialize, Deserialize)]
pub struct GetModlog {
  mod_user_id: Option<i32>,
  community_id: Option<i32>,
  page: Option<i64>,
  limit: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetModlogResponse {
  removed_posts: Vec<ModRemovePostView>,
  locked_posts: Vec<ModLockPostView>,
  stickied_posts: Vec<ModStickyPostView>,
  removed_comments: Vec<ModRemoveCommentView>,
  removed_communities: Vec<ModRemoveCommunityView>,
  banned_from_community: Vec<ModBanFromCommunityView>,
  banned: Vec<ModBanView>,
  added_to_community: Vec<ModAddCommunityView>,
  added: Vec<ModAddView>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSite {
  pub name: String,
  pub description: Option<String>,
  pub enable_downvotes: bool,
  pub open_registration: bool,
  pub enable_nsfw: bool,
  pub auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditSite {
  name: String,
  description: Option<String>,
  enable_downvotes: bool,
  open_registration: bool,
  enable_nsfw: bool,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSite {}

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteResponse {
  site: SiteView,
}

#[derive(Serialize, Deserialize)]
pub struct GetSiteResponse {
  site: Option<SiteView>,
  admins: Vec<UserView>,
  banned: Vec<UserView>,
  pub online: usize,
}

#[derive(Serialize, Deserialize)]
pub struct TransferSite {
  user_id: i32,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSiteConfig {
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSiteConfigResponse {
  config_hjson: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveSiteConfig {
  config_hjson: String,
  auth: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SiteView {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
  pub creator_id: i32,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub enable_downvotes: bool,
  pub open_registration: bool,
  pub enable_nsfw: bool,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub number_of_users: i64,
  pub number_of_posts: i64,
  pub number_of_comments: i64,
  pub number_of_communities: i64,
}


#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModRemoveCommentView {
  pub id: i32,
  pub mod_user_id: i32,
  pub comment_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub comment_user_id: i32,
  pub comment_user_name: String,
  pub comment_content: String,
  pub post_id: i32,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModRemovePostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModLockPostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub locked: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModStickyPostView {
  pub id: i32,
  pub mod_user_id: i32,
  pub post_id: i32,
  pub stickied: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub post_name: String,
  pub community_id: i32,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModRemoveCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub community_id: i32,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModBanFromCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub community_id: i32,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModBanView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModAddCommunityView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub community_id: i32,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
  pub community_name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ModAddView {
  pub id: i32,
  pub mod_user_id: i32,
  pub other_user_id: i32,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
  pub mod_user_name: String,
  pub other_user_name: String,
}