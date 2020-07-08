use crate::db::*;

/**
 * Categories for posts/content.
 */
#[derive(Debug, Deserialize, Serialize, PartialEq, Setters, CopyGetters)]
#[serde(rename_all = "camelCase")]
pub struct Category {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  #[serde(skip_serializing)]
  #[getset(get_copy = "pub with_prefix", set = "pub")]
  pub id: i64,
  pub name: String,
}

/**
 * Form for creating new category
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryForm {
  pub name: String,
}

impl Category {

  /// Dgraph type
  const GDB_TYPE: &'static str = "Category";

  /// Read all categories from DB
  pub async fn list_all(conn: &dgraph::Client) -> Result<Vec<Self>, Error> {
    list_nodes::<Category>(conn).await
  }
}

impl Node for Category {
  fn db_type_name() -> &'static str {
    Category::GDB_TYPE
  }
}

impl From<CategoryForm> for Category {
  // Default conversion
  fn from(form: CategoryForm) -> Self {
      Category { id: 0, name: form.name }
  }
}

impl Node for CategoryForm {
  fn db_type_name() -> &'static str {
    Category::GDB_TYPE
  }
}


// /***
//  * CRUD operations (use default impl.)
//  */
// #[async_trait]
// impl CrudNode<CategoryForm> for Category { }


