use super::*;

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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Category {

  // #[serde(serialize_with = "serialize_dgraph_id")]
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  pub id: i64,

  #[serde(rename(deserialize = "Category.name"))]
  pub name: String,

}

impl Category {
  const GDB_TYPE: &'static str = "Category";
}

/**
 * Result type for reading categories from DB
 */
#[derive(Debug, Deserialize)]
pub struct Categories {
  pub all: Vec<Category>,
}

/**
 * Form for creating new category
 */
#[derive(Debug, Clone)]
pub struct CategoryForm {
  pub name: String,
}

/**
 * Serialize form and add Dgraph type information
 */
impl Serialize for CategoryForm {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let mut state = serializer.serialize_struct(Category::GDB_TYPE, 3)?;
      state.serialize_field("Category.name", &self.name)?;
      state.serialize_field("uid", "_:x")?;
      state.serialize_field("dgraph.type", Category::GDB_TYPE)?;
      state.end()
  }
}


impl Category {

  const GDB_TYPE: &'static str = "Category";

  /// Read all categories from DB
  pub async fn list_all(conn: &dgraph::Client) -> Result<Vec<Self>> {

    let type_name = Category::GDB_TYPE;

    let q = format!(r#"
      categories(func: type({type_name})) {{
        expand(_all_)
      }}"#, type_name=type_name);

    let txn = conn.new_read_only_txn();
    let resp = txn.query(q).await?;

    let cats: Categories = resp.try_into()?;
    Ok(cats.all)
  }
}


#[async_trait]
impl Crud<CategoryForm> for Category {
 fn db_type_name(&self) -> &'static str {
   Category::GDB_TYPE
 }
}


