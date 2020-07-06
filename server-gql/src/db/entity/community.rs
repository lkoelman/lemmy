use crate::db::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
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


impl CommunityForm {
  const GDB_TYPE: &'static str = "Community";
}

impl Community {

  const GDB_TYPE: &'static str = "Community";

  /// Read all categories from DB
  pub async fn read_from_name(
      conn: &dgraph::Client,
      community_name: String) -> Result<Self> {

    let type_name = Community::GDB_TYPE;

    let q = format!(r#"
      categories(func: eq({t_name}.name, {name})) @filter(type({t_name})) {{
        expand(_all_)
      }}"#, t_name=type_name, name=community_name);

    let txn = conn.new_read_only_txn();
    let resp = txn.query(q).await?;

    let res: Community = resp.try_into()?;
    Ok(res)
  }

  pub fn get_url(&self) -> String {
    format!("https://{}/c/{}", Settings::get().hostname, self.name)
  }
}


#[async_trait]
impl CrudNode<CommunityForm> for Community {
 fn db_type_name(&self) -> &'static str {
   Community::GDB_TYPE
 }
}

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

impl Edge<CommunityModeratorForm> for CommunityModerator {
  fn from(&self) -> i64 {
      self.community_id
  }
  fn to(&self) -> i64 {
      self.user_id
  }
  fn from_form(form: &CommunityModeratorForm) -> Result<Self, Error> {
      Ok(CommunityModerator {
        community_id: form.community_id,
        user_id: form.user_id,
        published: chrono::Utc::now(),
      })
  }
  fn db_type_name() -> &'static str {
    "Community.Moderator"
  }
}

impl CommunityModerator {

  /**
   * Delete all moderators for community.
   */
  pub fn delete_for_community(conn: &dgraph::Client, for_community_id: i64) -> Result<usize> {
    // Edge is <Community> <moderated> <User>
    delete_edges(conn, Some(for_community_id), None, Self::db_type_name())
  }
}

impl CrudEdge<CommunityModeratorForm> for CommunityModerator {}

impl Joinable<CommunityModeratorForm> for CommunityModerator {

  /**
   * Add moderator for community
   */
  fn join(
    conn: &dgraph::Client,
    form: &CommunityModeratorForm,
  ) -> Result<Self> {

    let datetime: chrono::NaiveDateTime = chrono::Utc::now();

    let moderated = CommunityModerator {
      community_id: form.community_id,
      user_id: form.user_id,
      published: datetime,
    };

    let num_edges = create_edge::<CommunityModerator>(conn, moderated);

    match num_edges {
      Ok(i) => Ok(moderated),
      Err(e) => e
    }
  }

  /**
   * Delete moderator for community
   */
  fn leave(
    conn: &dgraph::Client,
    form: &CommunityModeratorForm,
  ) -> Result<usize> {
    // Edge is <Community> <moderated> <User>
    delete_edges(conn,
      Some(form.from()),
      Some(form.to()),
      Self::db_type_name())
  }

}


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

impl Edge<CommunityUserBanForm> for CommunityUserBan {
  fn from(&self) -> i64 {
      self.community_id
  }
  fn to(&self) -> i64 {
      self.user_id
  }
  fn from_form(form: &CommunityUserBanForm) -> Result<Self, Error> {
    Ok(CommunityUserBan {
      community_id: form.community_id,
      user_id: form.user_id,
      published: chrono::Utc::now(),
    })
  }
  fn db_type_name() -> &'static str {
    "Community.BanUser"
  }
}

impl Bannable<CommunityUserBanForm> for CommunityUserBan {

  /**
   * Ban user from community
   */
  fn ban(
    conn: &dgraph::Client,
    form: &CommunityUserBanForm,
  ) -> Result<Self> {

    let datetime: chrono::NaiveDateTime = chrono::Utc::now();

    let ban = CommunityUserBan {
      community_id: form.community_id,
      user_id: form.user_id,
      published: datetime,
    };

    let num_edges = create_edge::<CommunityUserBan>(conn, ban);

    match num_edges {
      Ok(i) => Ok(ban),
      Err(e) => e
    }
  }

  /**
   * Unban user from community
   */
  fn unban(
    conn: &dgraph::Client,
    form: &CommunityUserBanForm,
  ) -> Result<usize> {

    // Edge is <Community> <Ban> <User>
    delete_edges(conn,
      Some(form.from()),
      Some(form.to()),
      Self::db_type_name())
  }
}

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

impl Edge<CommunityFollowerForm> for CommunityFollower {
  fn from(&self) -> i64 {
      self.community_id
  }
  fn to(&self) -> i64 {
      self.user_id
  }
  fn from_form(form: &CommunityFollowerForm) -> Result<Self, Error> {
      Ok(CommunityFollower {
        community_id: form.community_id,
        user_id: form.user_id,
        published: chrono::Utc::now(),
      })
  }
  fn db_type_name() -> &'static str {
    "Follower"
  }
}

impl Followable<CommunityFollowerForm> for CommunityFollower {

  /**
   * Add user as follower for community.
   */
  fn follow(
    conn: &dgraph::Client,
    form: &CommunityFollowerForm,
  ) -> Result<Self> {

    let datetime: chrono::NaiveDateTime = chrono::Utc::now();

    let follower = CommunityFollower {
      community_id: form.community_id,
      user_id: form.user_id,
      published: datetime,
    };

    let num_edges = create_edge::<CommunityFollower>(conn, follower);

    match num_edges {
      Ok(i) => Ok(follower),
      Err(e) => e
    }
  }

  /**
   * Remove user as follower for community
   */
  fn ignore(
    conn: &dgraph::Client,
    form: &CommunityFollowerForm,
  ) -> Result<usize> {

    // Edge is <Community> <Follower> <User>
    delete_edges(conn,
      Some(form.from()),
      Some(form.to()),
      Self::db_type_name())
  }
}