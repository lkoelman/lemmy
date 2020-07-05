use super::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Community {
  pub id: i32,
  pub name: String,
  pub title: String,
  pub description: Option<String>,
  pub category_id: i32,
  pub creator_id: i32,
  pub removed: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub nsfw: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommunityForm {
  pub name: String,
  pub title: String,
  pub description: Option<String>,
  pub category_id: i32,
  pub creator_id: i32,
  pub removed: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
  pub nsfw: bool,
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
impl Crud<CommunityForm> for Community {
 fn db_type_name(&self) -> &'static str {
   Community::GDB_TYPE
 }
}

//#############################################################################

#[derive(PartialEq, Debug)]
pub struct CommunityModerator {
  pub id: i32,
  pub community_id: i32,
  pub user_id: i32,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommunityModeratorForm {
  pub community_id: i32,
  pub user_id: i32,
}

impl CommunityModerator {
  pub fn delete_for_community(conn: &dgraph::Client, for_community_id: i32) -> Result<usize> {

    let mut txn = conn.new_mutated_txn();
    let mut mu = Mutation::new();
    mu.set_delete_nquads(format!(r#"uid({}) * * ."#,
                                 for_community_id))

    let resp = txn.mutate(mu).await?;
    txn.commit().await? // .expect("Transaction is commited");

    Ok(resp.uids.len())
  }
}

impl Joinable<CommunityModeratorForm> for CommunityModerator {

  fn join(
    conn: &dgraph::Client,
    community_user_form: &CommunityModeratorForm,
  ) -> Result<Self> {
    use crate::schema::community_moderator::dsl::*;
    insert_into(community_moderator)
      .values(community_user_form)
      .get_result::<Self>(conn)
  }

  fn leave(
    conn: &dgraph::Client,
    community_user_form: &CommunityModeratorForm,
  ) -> Result<usize> {
    use crate::schema::community_moderator::dsl::*;
    diesel::delete(
      community_moderator
        .filter(community_id.eq(community_user_form.community_id))
        .filter(user_id.eq(community_user_form.user_id)),
    )
    .execute(conn)
  }

}


//#############################################################################

#[derive(PartialEq, Debug)]
#[belongs_to(Community)]
pub struct CommunityUserBan {
  pub id: i32,
  pub community_id: i32,
  pub user_id: i32,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommunityUserBanForm {
  pub community_id: i32,
  pub user_id: i32,
}

impl Bannable<CommunityUserBanForm> for CommunityUserBan {
  fn ban(
    conn: &dgraph::Client,
    community_user_ban_form: &CommunityUserBanForm,
  ) -> Result<Self> {
    use crate::schema::community_user_ban::dsl::*;
    insert_into(community_user_ban)
      .values(community_user_ban_form)
      .get_result::<Self>(conn)
  }

  fn unban(
    conn: &dgraph::Client,
    community_user_ban_form: &CommunityUserBanForm,
  ) -> Result<usize> {
    use crate::schema::community_user_ban::dsl::*;
    diesel::delete(
      community_user_ban
        .filter(community_id.eq(community_user_ban_form.community_id))
        .filter(user_id.eq(community_user_ban_form.user_id)),
    )
    .execute(conn)
  }
}

//#############################################################################

#[derive(PartialEq, Debug)]
#[belongs_to(Community)]
pub struct CommunityFollower {
  pub id: i32,
  pub community_id: i32,
  pub user_id: i32,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommunityFollowerForm {
  pub community_id: i32,
  pub user_id: i32,
}

impl Followable<CommunityFollowerForm> for CommunityFollower {
  fn follow(
    conn: &dgraph::Client,
    community_follower_form: &CommunityFollowerForm,
  ) -> Result<Self> {
    use crate::schema::community_follower::dsl::*;
    insert_into(community_follower)
      .values(community_follower_form)
      .get_result::<Self>(conn)
  }
  fn ignore(
    conn: &dgraph::Client,
    community_follower_form: &CommunityFollowerForm,
  ) -> Result<usize> {
    use crate::schema::community_follower::dsl::*;
    diesel::delete(
      community_follower
        .filter(community_id.eq(&community_follower_form.community_id))
        .filter(user_id.eq(&community_follower_form.user_id)),
    )
    .execute(conn)
  }
}