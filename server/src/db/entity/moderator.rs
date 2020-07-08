use crate::db::*;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRemovePost {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub post_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModRemovePostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

impl Edge for ModRemovePost {
  fn from(&self) -> i64 {
    self.post_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "Post.ModRemove"
  }
}

impl From<ModRemovePostForm> for ModRemovePost {
  fn from(form: ModRemovePostForm) -> Self {
    ModRemovePost {
      mod_user_id: form.mod_user_id,
      post_id: form.post_id,
      reason: form.reason,
      removed: form.removed,
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModRemovePostForm> for ModRemovePost {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLockPost {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub post_id: i64,
  pub locked: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModLockPostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub locked: Option<bool>,
}

impl Edge for ModLockPost {
  fn from(&self) -> i64 {
    self.post_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "Post.ModLock"
  }
}

impl From<ModLockPostForm> for ModLockPost {
  fn from(form: ModLockPostForm) -> Self {
    ModLockPost {
      mod_user_id: form.mod_user_id,
      post_id: form.post_id,
      locked: form.locked,
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModLockPostForm> for ModLockPost {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModStickyPost {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub post_id: i64,
  pub stickied: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModStickyPostForm {
  pub mod_user_id: i64,
  pub post_id: i64,
  pub stickied: Option<bool>,
}

impl Edge for ModStickyPost {
  fn from(&self) -> i64 {
    self.post_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "Post.ModSticky"
  }
}

impl From<ModStickyPostForm> for ModStickyPost {
  fn from(form: ModStickyPostForm) -> Self {
    ModStickyPost {
      mod_user_id: form.mod_user_id,
      post_id: form.post_id,
      stickied: form.stickied,
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModStickyPostForm> for ModStickyPost {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRemoveComment {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub comment_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModRemoveCommentForm {
  pub mod_user_id: i64,
  pub comment_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

impl Edge for ModRemoveComment {
  fn from(&self) -> i64 {
    self.comment_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "Comment.ModRemove"
  }
}

impl From<ModRemoveCommentForm> for ModRemoveComment {
  fn from(form: ModRemoveCommentForm) -> Self {
    ModRemoveComment {
      mod_user_id: form.mod_user_id,
      comment_id: form.comment_id,
      reason: form.reason,
      removed: form.removed,
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModRemoveCommentForm> for ModRemoveComment {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRemoveCommunity {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub community_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModRemoveCommunityForm {
  pub mod_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

impl Edge for ModRemoveCommunity {
  fn from(&self) -> i64 {
    self.community_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "Community.ModRemove"
  }
}

impl From<ModRemoveCommunityForm> for ModRemoveCommunity {
  fn from(form: ModRemoveCommunityForm) -> Self {
    ModRemoveCommunity {
      mod_user_id: form.mod_user_id,
      community_id: form.community_id,
      reason: form.reason,
      removed: form.removed,
      expires: Some(
        chrono::Utc::now().naive_utc() + chrono::Duration::days(365)),
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModRemoveCommunityForm> for ModRemoveCommunity {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct ModBanFromCommunity {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub other_user_id: i64,
  #[serde(skip)]
  pub community_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModBanFromCommunityForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}


impl ModBanFromCommunity {
  /// Dgraph type
  const GDB_TYPE: &'static str = "ModBanFromCommunity";
}

impl Node for ModBanFromCommunity {
  fn db_type_name() -> &'static str {
    ModBanFromCommunity::GDB_TYPE
  }
}

impl From<ModBanFromCommunityForm> for ModBanFromCommunity {
  // Default conversion
  fn from(form: ModBanFromCommunityForm) -> Self {
      ModBanFromCommunity {
        id: 0,
        mod_user_id: form.mod_user_id,
        other_user_id: form.other_user_id,
        community_id: form.community_id,
        reason: form.reason,
        banned: form.banned,
        expires: form.expires,
        when_: chrono::Utc::now().naive_utc()
      }
  }
}

// impl CrudNode<ModBanFromCommunityForm> for ModBanFromCommunity {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModBan {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub other_user_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModBanForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

impl Edge for ModBan {
  fn from(&self) -> i64 {
    self.other_user_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "User.ModBan"
  }
}

impl From<ModBanForm> for ModBan {
  fn from(form: ModBanForm) -> Self {
    ModBan {
      mod_user_id: form.mod_user_id,
      other_user_id: form.other_user_id,
      reason: form.reason,
      banned: form.banned,
      expires: Some(
        chrono::Utc::now().naive_utc() + chrono::Duration::days(365)),
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModBanForm> for ModBan {}

//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct ModAddCommunity {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub other_user_id: i64,
  #[serde(skip)]
  pub community_id: i64,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModAddCommunityForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub community_id: i64,
  pub removed: Option<bool>,
}

// TODO: add this triple as node
impl ModAddCommunity {
  /// Dgraph type
  const GDB_TYPE: &'static str = "ModAddCommunity";
}

impl Node for ModAddCommunity {
  fn db_type_name() -> &'static str {
    ModAddCommunity::GDB_TYPE
  }
}

impl From<ModAddCommunityForm> for ModAddCommunity {
  // Default conversion
  fn from(form: ModAddCommunityForm) -> Self {
      ModAddCommunity {
        id: 0,
        mod_user_id: form.mod_user_id,
        other_user_id: form.other_user_id,
        community_id: form.community_id,
        removed: form.removed,
        when_: chrono::Utc::now().naive_utc()
      }
  }
}

// impl CrudNode<ModAddCommunityForm> for ModAddCommunity {}


//#############################################################################

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAdd {
  #[serde(skip)]
  pub mod_user_id: i64,
  #[serde(skip)]
  pub other_user_id: i64,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ModAddForm {
  pub mod_user_id: i64,
  pub other_user_id: i64,
  pub removed: Option<bool>,
}

impl Edge for ModAdd {
  fn from(&self) -> i64 {
    self.other_user_id
  }
  fn to(&self) -> i64 {
    self.mod_user_id
  }
  fn db_type_name() -> &'static str {
    "User.ModAdd"
  }
}

impl From<ModAddForm> for ModAdd {
  fn from(form: ModAddForm) -> Self {
    ModAdd {
      mod_user_id: form.mod_user_id,
      other_user_id: form.other_user_id,
      removed: form.removed,
      when_: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<ModAddForm> for ModAdd {}
