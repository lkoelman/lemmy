use super::post::Post;
use super::*;
use crate::schema::{comment, comment_like, comment_saved};


#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Comment {
  pub id: i32,
  pub creator_id: i32,
  pub post_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: bool,
  pub read: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
}

#[derive(Clone)]
pub struct CommentForm {
  pub creator_id: i32,
  pub post_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub removed: Option<bool>,
  pub read: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
}


#[async_trait]
impl Crud<CommentForm> for Comment {
  // use default implementations
  fn db_type_name(&self) -> &'static str {
    "Comment"
  }
}

// ############################################################################
// Likes
// ############################################################################


#[derive(PartialEq, Debug, Clone)]
pub struct CommentLike {
  pub id: i32,
  pub user_id: i32,
  pub comment_id: i32,
  pub post_id: i32,
  pub score: i16,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommentLikeForm {
  pub user_id: i32,
  pub comment_id: i32,
  pub post_id: i32,
  pub score: i16,
}

// helper trait for default implementations to work
impl Like for CommentLikeForm {
  fn from(&self) -> i32 { self.user_id }
  fn to(&self) -> i32 { self.comment_id }
  fn score(&self) -> i16 { self.score }
}

impl CommentLike {

  const GDB_TYPE: &'static str = "CommentLike";

  pub fn from_post(conn: &dgraph::Client, post_id_from: i32) -> Result<Vec<Self>> {
    use crate::schema::comment_like::dsl::*;
    comment_like
      .filter(post_id.eq(post_id_from))
      .load::<Self>(conn)

    // TODO: either reverse query (direction of edges) or mark edges reversed in schema definition
    let q = format!(r#"
      node(func: has({edge_name})) @filter(type({user_type})){
        uid
        score @facets
        ~{comment_type} {
          uid
          ~{post_type} {
            uid
          }
        }
      }"#,
      user_type=User::db_type_name(),
      comment_type=Comment::db_type_name(),
      edge_name=CommentLike::db_type_name(),
          type_name);

    let txn = conn.new_read_only_txn()
    let resp = txn.query(q).await?;

    resp.try_into::<CommentLike>()?
  }
}

impl Likeable<CommentLikeForm> for CommentLike {
  // use default implementations
  fn db_type_name() -> &'static str {
    CommentLike::GDB_TYPE
  }
}


// ############################################################################
// Saving comments
// ############################################################################


#[derive(PartialEq, Debug)]
#[belongs_to(Comment)]
pub struct CommentSaved {
  pub id: i32,
  pub comment_id: i32,
  pub user_id: i32,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct CommentSavedForm {
  pub comment_id: i32,
  pub user_id: i32,
}

impl Saveable<CommentSavedForm> for CommentSaved {
  fn save(conn: &dgraph::Client, comment_saved_form: &CommentSavedForm) -> Result<Self> {
    use crate::schema::comment_saved::dsl::*;
    insert_into(comment_saved)
      .values(comment_saved_form)
      .get_result::<Self>(conn)
  }
  fn unsave(conn: &dgraph::Client, comment_saved_form: &CommentSavedForm) -> Result<usize> {
    use crate::schema::comment_saved::dsl::*;
    diesel::delete(
      comment_saved
        .filter(comment_id.eq(comment_saved_form.comment_id))
        .filter(user_id.eq(comment_saved_form.user_id)),
    )
    .execute(conn)
  }
}
