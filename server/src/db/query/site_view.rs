use crate::db::*;



#[derive(
  PartialEq, Debug, Serialize, Deserialize, Clone,
)]
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

impl SiteView {
  pub fn read(conn: &dgraph_tonic::Client) -> Result<Self, Error> {
    use super::site_view::site_view::dsl::*;
    site_view.first::<Self>(conn)
  }
}
