use super::post::Post;
use crate::db::*;


#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  #[serde(rename(deserialize = "uid"))]
  pub id: i64,
  pub creator_id: i64,
  pub post_id: i64,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentForm {
  pub creator_id: i64,
  pub post_id: i64,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: Option<bool>,
  pub read: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
}

impl Comment {
  /// Dgraph type
  const GDB_TYPE: &'static str = "Comment";
}

impl Node for Comment {
  fn db_type_name() -> &'static str {
    Comment::GDB_TYPE
  }
}

impl Node for CommentForm {
  fn db_type_name() -> &'static str {
    Comment::GDB_TYPE
  }
}

#[async_trait]
impl CrudNode<CommentForm> for Comment { }

// ############################################################################
// Likes
// ############################################################################


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentLike {
  #[serde(skip)]
  pub user_id: i64,
  #[serde(skip)]
  pub comment_id: i64,
  pub score: i16,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommentLikeForm {
  pub user_id: i64,
  pub comment_id: i64,
  pub score: i16,
}

/**
 * Result type for deserializing comment likes from DB
 */
#[derive(Debug, Deserialize)]
pub struct CommentLikesResult {
  // TODO: serde attribute flatten?
  pub all: Vec<CommentLike>,
}

impl Edge<CommentLikeForm> for CommentLike {
  fn from(&self) -> i64 {
    self.comment_id
  }
  fn to(&self) -> i64 {
      self.user_id
  }
  fn from_form(form: &CommentLikeForm) -> Result<Self, Error> {
      Ok(CommentLike {
        comment_id: form.comment_id,
        user_id: form.user_id,
        score: form.score,
        published: chrono::Utc::now(),
      })
  }
  fn db_type_name() -> &'static str {
    CommentLike::GDB_TYPE
  }
}


impl CommentLike {

  /// DB type name
  const GDB_TYPE: &'static str = "Comment.UserLike";

  /**
   * Get all comment likes for post
   */
  pub fn from_post(conn: &dgraph::Client, post_id_from: i32) -> Result<Vec<Self>> {

    let q = format!(r#"
      node(func: has({p_post_comment})) @filter(type({t_post})){
        uid
        {t_comment} {
          uid
          {t_commentlike} {
            score @ facets
            uid
          }
        }
      }"#,
      p_post_comment=format!("{}.{}", Post::db_type_name(), Comment::db_type_name())
      t_post=Post::db_type_name(),
      t_comment=Comment::db_type_name(),
      t_commentlike=CommentLike::db_type_name());

    let txn = conn.new_read_only_txn();
    let resp = txn.query(q).await?;

    let likes: CommentLikesResult = resp.try_into()?;
    Ok(likes.all)
  }
}

impl Likeable<CommentLikeForm> for CommentLike {}


// ############################################################################
// Saving comments
// ############################################################################


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSaved {
  #[serde(skip)]
  pub comment_id: i64,
  #[serde(skip)]
  pub user_id: i64,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommentSavedForm {
  pub comment_id: i64,
  pub user_id: i64,
}

impl Edge<CommentSavedForm> for CommentSaved {
  fn from(&self) -> i64 {
    self.user_id
  }
  fn to(&self) -> i64 {
      self.comment_id
  }
  fn from_form(form: &CommentSavedForm) -> Result<Self, Error> {
      Ok(CommentSaved {
        comment_id: form.comment_id,
        user_id: form.user_id,
        published: chrono::Utc::now(),
      })
  }
  fn db_type_name() -> &'static str {
    "User.SavedComment"
  }
}

impl Saveable<CommentSavedForm> for CommentSaved {}
