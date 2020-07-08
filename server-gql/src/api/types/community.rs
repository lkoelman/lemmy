use serde::{Serialize, Deserialize};
use crate::db::{community_view::*, user_view::*};


#[derive(Serialize, Deserialize)]
pub struct GetCommunity {
  id: Option<i32>,
  name: Option<String>,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetCommunityResponse {
  pub community: CommunityView,
  moderators: Vec<CommunityModeratorView>,
  admins: Vec<UserView>,
  pub online: usize,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCommunity {
  name: String,
  title: String,
  description: Option<String>,
  category_id: i32,
  nsfw: bool,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommunityResponse {
  pub community: CommunityView,
}

#[derive(Serialize, Deserialize)]
pub struct ListCommunities {
  sort: String,
  page: Option<i64>,
  limit: Option<i64>,
  auth: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListCommunitiesResponse {
  communities: Vec<CommunityView>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BanFromCommunity {
  pub community_id: i32,
  user_id: i32,
  ban: bool,
  reason: Option<String>,
  expires: Option<i64>,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BanFromCommunityResponse {
  user: UserView,
  banned: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AddModToCommunity {
  pub community_id: i32,
  user_id: i32,
  added: bool,
  auth: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddModToCommunityResponse {
  moderators: Vec<CommunityModeratorView>,
}

#[derive(Serialize, Deserialize)]
pub struct EditCommunity {
  pub edit_id: i32,
  name: String,
  title: String,
  description: Option<String>,
  category_id: i32,
  removed: Option<bool>,
  deleted: Option<bool>,
  nsfw: bool,
  reason: Option<String>,
  expires: Option<i64>,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct FollowCommunity {
  community_id: i32,
  follow: bool,
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetFollowedCommunities {
  auth: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetFollowedCommunitiesResponse {
  communities: Vec<CommunityFollowerView>,
}

#[derive(Serialize, Deserialize)]
pub struct TransferCommunity {
  community_id: i32,
  user_id: i32,
  auth: String,
}
