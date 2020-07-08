use serde::{Serialize, Deserialize};
use crate::db::comment_view::*;

#[derive(Serialize, Deserialize)]
pub struct CreateComment {
  content: String,
  parent_id: Option<i32>,
  edit_id: Option<i32>, // TODO this isn't used
  pub post_id: i32,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditComment {
  content: String,
  parent_id: Option<i32>, // TODO why are the parent_id, creator_id, post_id, etc fields required? They aren't going to change
  edit_id: i32,
  creator_id: i32,
  pub post_id: i32,
  removed: Option<bool>,
  deleted: Option<bool>,
  reason: Option<String>,
  read: Option<bool>,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveComment {
  comment_id: i32,
  save: bool,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommentResponse {
  pub comment: CommentView,
  pub recipient_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCommentLike {
  comment_id: i32,
  pub post_id: i32,
  score: i16,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetComments {
  type_: String,
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  pub community_id: Option<i32>,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetCommentsResponse {
  comments: Vec<CommentView>,
}
