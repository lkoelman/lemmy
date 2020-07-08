use crate::db::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct Site {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
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
#[serde(rename_all = "camelCase")]
pub struct SiteForm {
  pub name: String,
  pub description: Option<String>,
  pub creator_id: i64,
  pub updated: Option<chrono::NaiveDateTime>,
  pub enable_downvotes: bool,
  pub open_registration: bool,
  pub enable_nsfw: bool,
}

impl Site {
  /// DB type name
  const GDB_TYPE: &'static str = "Site";
}

impl Node for Site {
  fn db_type_name() -> &'static str {
    Site::GDB_TYPE
  }
}

impl From<SiteForm> for Site {
  fn from(form: SiteForm) -> Self {
      Site {
        id: 0,
        name: form.name,
        description: form.description,
        creator_id: form.creator_id,
        updated: form.updated,
        enable_downvotes: form.enable_downvotes,
        open_registration: form.open_registration,
        enable_nsfw: form.enable_downvotes,
        published: chrono::Utc::now().naive_utc(),
      }
  }
}

impl Node for SiteForm {
  fn db_type_name() -> &'static str {
    Site::GDB_TYPE
  }
}

// /**
//  * CRUD operations for sites
//  */
// #[async_trait]
// impl CrudNode<SiteForm> for Site {}
