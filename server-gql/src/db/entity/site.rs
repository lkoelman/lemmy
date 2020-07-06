use super::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Site {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub creator_id: i64,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub enable_downvotes: bool,
  pub open_registration: bool,
  pub enable_nsfw: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SiteForm {
  pub name: String,
  pub description: Option<String>,
  pub creator_id: i64,
  pub updated: Option<chrono::NaiveDateTime>,
  pub enable_downvotes: bool,
  pub open_registration: bool,
  pub enable_nsfw: bool,
}

/**
 * CRUD operations for sites
 */
#[async_trait]
impl CrudNode<SiteForm> for Site {  
  fn db_type_name(&self) -> &'static str {
    "Site"
  }
}
