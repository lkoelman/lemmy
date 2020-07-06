use crate::db::*;

// fn serialize_dgraph_id<S>(x: &i64, s: S) -> std::result::Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     s.serialize_str("_:x")
// }

// fn deserialize_dgraph_id<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     i64::deserialize(deserializer)
// }

/**
 * Categories for posts/content.
 */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {

  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  pub id: i64,

  // #[serde(rename(deserialize = "Category.name"))]
  pub name: String,

}

/**
 * Result type for deserializing categories from DB
 */
#[derive(Debug, Deserialize)]
pub struct Categories {
  pub all: Vec<Category>,
}

/**
 * Form for creating new category
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryForm {
  pub name: String,
}

// /**
//  * Serialize form and add Dgraph type information
//  */
// impl Serialize for CategoryForm {
//   fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//   where
//       S: Serializer,
//   {
//       let mut state = serializer.serialize_struct(Category::GDB_TYPE, 3)?;
//       state.serialize_field("Category.name", &self.name)?;
//       state.serialize_field("uid", "_:x")?;
//       state.serialize_field("dgraph.type", Category::GDB_TYPE)?;
//       state.end()
//   }
// }

impl Category {

  /// Dgraph type
  const GDB_TYPE: &'static str = "Category";

  /// Read all categories from DB
  pub async fn list_all(conn: &dgraph::Client) -> Result<Vec<Self>, Error> {
    let cats = list_nodes::<Categories>(conn);
    Ok(cats.all)
  }
}

impl Node for Category {
  fn db_type_name() -> &'static str {
    Category::GDB_TYPE
  }
}

impl Node for CategoryForm {
  fn db_type_name() -> &'static str {
    Category::GDB_TYPE
  }
}


/***
 * CRUD operations (use default impl.)
 */
#[async_trait]
impl CrudNode<CategoryForm> for Category { }


