use crate::db::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct Community {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub name: String,
  pub title: String,
  pub description: Option<String>,
  pub category_id: i64,
  pub creator_id: i64,
  pub removed: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub nsfw: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityForm {
  pub name: String,
  pub title: String,
  pub description: Option<String>,
  pub category_id: i64,
  pub creator_id: i64,
  pub removed: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
  pub nsfw: bool,
}

impl Node for Community {
  fn db_type_name() -> &'static str {
    Community::GDB_TYPE
  }
}

impl From<CommunityForm> for Community {
  fn from(form: CommunityForm) -> Self {
      Community {
        id: 0,
        name: form.name,
        title: form.title,
        description: form.description,
        category_id: form.category_id,
        creator_id: form.creator_id,
        removed: form.removed.unwrap_or(false),
        updated: form.updated,
        deleted: form.deleted.unwrap_or(false),
        nsfw: form.nsfw,
        published: chrono::Utc::now().naive_utc()
      }
  }
}

impl Node for CommunityForm {
  fn db_type_name() -> &'static str {
    Community::GDB_TYPE
  }
}

impl Community {
  /// DB type name
  const GDB_TYPE: &'static str = "Community";

  /// Read all categories from DB
  pub async fn read_from_name(
    conn: &dgraph::Client,
    community_name: String,
  ) -> Result<Self, Error> {
    let type_name = Community::GDB_TYPE;

    let q = format!(
      r#"
      community(func: eq({t_name}.name, {name})) @filter(type({t_name})) {{
        expand(_all_)
      }}"#,
      t_name = type_name,
      name = community_name
    );

    let txn = conn.new_read_only_txn();
    let resp = txn.query(q).await?;

    let res: Community = resp.try_into()?;
    Ok(res)
  }

  pub fn get_url(&self) -> String {
    format!("https://{}/c/{}", Settings::get().hostname, self.name)
  }
}

// #[async_trait]
// impl CrudNode<CommunityForm> for Community {}

//#############################################################################

// NOTE: types implemented as edges
//  - have no UID (id)
//  - do not serialize the 'to' and 'from' fields of the form

/// Core type also used as DB form
#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityModerator {
  #[serde(skip)]
  pub community_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

/// Form for internal use
#[derive(Clone, Debug)]
pub struct CommunityModeratorForm {
  pub community_id: i64,
  pub user_id: i64,
}

impl Edge for CommunityModerator {
  fn from(&self) -> i64 {
    self.community_id
  }
  fn to(&self) -> i64 {
    self.user_id
  }
  fn db_type_name() -> &'static str {
    "Community.Moderator"
  }
}

impl From<CommunityModeratorForm> for CommunityModerator {
  fn from(form: CommunityModeratorForm) -> Self {
    CommunityModerator {
      community_id: form.community_id,
      user_id: form.user_id,
      published: chrono::Utc::now().naive_utc(),
    }
  }
}

impl CommunityModerator {
  /**
   * Delete all moderators for community.
   */
  pub async fn delete_for_community(
    conn: &dgraph::Client,
    for_community_id: i64,
  ) -> Result<usize, Error> {
    // Edge is <Community> <moderated> <User>
    delete_edges(
      conn,
      Some(for_community_id),
      None,
      Some(Self::db_type_name()),
    )
    .await
  }
}

// impl CrudEdge<CommunityModeratorForm> for CommunityModerator {}

//#############################################################################

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityUserBan {
  #[serde(skip)]
  pub community_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommunityUserBanForm {
  pub community_id: i64,
  pub user_id: i64,
}

impl Edge for CommunityUserBan {
  fn from(&self) -> i64 {
    self.community_id
  }
  fn to(&self) -> i64 {
    self.user_id
  }
  fn db_type_name() -> &'static str {
    "Community.BanUser"
  }
}

impl From<CommunityUserBanForm> for CommunityUserBan {
  fn from(form: CommunityUserBanForm) -> Self {
    CommunityUserBan {
      community_id: form.community_id,
      user_id: form.user_id,
      published: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<CommunityUserBanForm> for CommunityUserBan {}

//#############################################################################

/// Core type also used as DB form
#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityFollower {
  #[serde(skip)]
  pub community_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

/// Form for internal use
#[derive(Clone)]
pub struct CommunityFollowerForm {
  pub community_id: i64,
  pub user_id: i64,
}

impl Edge for CommunityFollower {
  fn from(&self) -> i64 {
    self.community_id
  }
  fn to(&self) -> i64 {
    self.user_id
  }
  fn db_type_name() -> &'static str {
    "Follower"
  }
}

impl From<CommunityFollowerForm> for CommunityFollower {
  fn from(form: CommunityFollowerForm) -> Self {
    CommunityFollower {
      community_id: form.community_id,
      user_id: form.user_id,
      published: chrono::Utc::now().naive_utc(),
    }
  }
}

// impl CrudEdge<CommunityFollowerForm> for CommunityFollower {}
