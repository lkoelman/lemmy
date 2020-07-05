use serde::{Serialize, Deserialize};
use super::community::{CommunityFollowerView, CommunityModeratorView};
use super::post::{PostView};
use super::comment::{ReplyView, CommentView};

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
  username_or_email: String,
  password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Register {
  pub username: String,
  pub email: Option<String>,
  pub password: String,
  pub password_verify: String,
  pub admin: bool,
  pub show_nsfw: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SaveUserSettings {
  show_nsfw: bool,
  theme: String,
  default_sort_type: i16,
  default_listing_type: i16,
  lang: String,
  avatar: Option<String>,
  email: Option<String>,
  matrix_user_id: Option<String>,
  new_password: Option<String>,
  new_password_verify: Option<String>,
  old_password: Option<String>,
  show_avatars: bool,
  send_notifications_to_email: bool,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
  pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserDetails {
  user_id: Option<i32>,
  username: Option<String>,
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  community_id: Option<i32>,
  saved_only: bool,
  auth: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct GetUserDetailsResponse {
  user: UserView,
  follows: Vec<CommunityFollowerView>,
  moderates: Vec<CommunityModeratorView>,
  comments: Vec<CommentView>,
  posts: Vec<PostView>,
  admins: Vec<UserView>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRepliesResponse {
  replies: Vec<ReplyView>,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserMentionsResponse {
  mentions: Vec<UserMentionView>,
}

#[derive(Serialize, Deserialize)]
pub struct MarkAllAsRead {
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddAdmin {
  user_id: i32,
  added: bool,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddAdminResponse {
  admins: Vec<UserView>,
}

#[derive(Serialize, Deserialize)]
pub struct BanUser {
  user_id: i32,
  ban: bool,
  reason: Option<String>,
  expires: Option<i64>,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BanUserResponse {
  user: UserView,
  banned: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetReplies {
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  unread_only: bool,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserMentions {
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  unread_only: bool,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditUserMention {
  user_mention_id: i32,
  read: Option<bool>,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMentionResponse {
  mention: UserMentionView,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAccount {
  password: String,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
  email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordResetResponse {}

#[derive(Serialize, Deserialize)]
pub struct PasswordChange {
  token: String,
  password: String,
  password_verify: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePrivateMessage {
  content: String,
  pub recipient_id: i32,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditPrivateMessage {
  edit_id: i32,
  content: Option<String>,
  deleted: Option<bool>,
  read: Option<bool>,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPrivateMessages {
  unread_only: bool,
  page: Option<i64>,
  limit: Option<i64>,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateMessagesResponse {
  messages: Vec<PrivateMessageView>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateMessageResponse {
  message: PrivateMessageView,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserJoin {
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserJoinResponse {
  pub user_id: i32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UserView {
  pub id: i32,
  pub name: String,
  pub avatar: Option<String>,
  pub email: Option<String>,
  pub matrix_user_id: Option<String>,
  pub fedi_name: String,
  pub admin: bool,
  pub banned: bool,
  pub show_avatars: bool,
  pub send_notifications_to_email: bool,
  pub published: chrono::NaiveDateTime,
  pub number_of_posts: i64,
  pub post_score: i64,
  pub number_of_comments: i64,
  pub comment_score: i64,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UserMentionView {
  pub id: i32,
  pub user_mention_id: i32,
  pub creator_id: i32,
  pub post_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub community_id: i32,
  pub community_name: String,
  pub banned: bool,
  pub banned_from_community: bool,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub score: i64,
  pub upvotes: i64,
  pub downvotes: i64,
  pub hot_rank: i32,
  pub user_id: Option<i32>,
  pub my_vote: Option<i32>,
  pub saved: Option<bool>,
  pub recipient_id: i32,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PrivateMessageView {
  pub id: i32,
  pub creator_id: i32,
  pub recipient_id: i32,
  pub content: String,
  pub deleted: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub creator_name: String,
  pub creator_avatar: Option<String>,
  pub recipient_name: String,
  pub recipient_avatar: Option<String>,
}