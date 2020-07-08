use serde::{Serialize, Deserialize};
use crate::db::{community_view::*, comment_view::*, user_view::*, post_view::*};


#[derive(Serialize, Deserialize)]
pub struct CreatePost {
  name: String,
  url: Option<String>,
  body: Option<String>,
  nsfw: bool,
  pub community_id: i32,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostResponse {
  pub post: PostView,
}

#[derive(Serialize, Deserialize)]
pub struct GetPost {
  pub id: i32,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPostResponse {
  post: PostView,
  comments: Vec<CommentView>,
  community: CommunityView,
  moderators: Vec<CommunityModeratorView>,
  admins: Vec<UserView>,
  pub online: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GetPosts {
  type_: String,
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  pub community_id: Option<i32>,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPostsResponse {
  posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostLike {
  post_id: i32,
  score: i16,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditPost {
  pub edit_id: i32,
  creator_id: i32,
  community_id: i32,
  name: String,
  url: Option<String>,
  body: Option<String>,
  removed: Option<bool>,
  deleted: Option<bool>,
  nsfw: bool,
  locked: Option<bool>,
  stickied: Option<bool>,
  reason: Option<String>,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct SavePost {
  post_id: i32,
  save: bool,
  auth: String,
}
