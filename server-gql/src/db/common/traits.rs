use crate::db::*;
use super::dgraph_utils::*;

/**
 * Generic CRUD operation (create, read, update, delete)
 * for any DB type.
 */
#[async_trait]
pub trait CrudNode<F> 
where
  Self : Sized + DeserializeOwned + Node,
  F : Sized + Serialize + Node {

  /// Get the DB type
  fn db_type_name() -> &'static str;

  /// Read DB type : default implementation
  async fn read(conn: &'_ dgraph::Client, id: i64) -> Result<Self, Error> {
    read_node::<Self>(conn, id, Self::db_type_name())
  }

  /// Delete DB type : default implementation
  async fn delete(conn: &'_ dgraph::Client, id: i64) -> Result<usize, Error> {
    delete_node(conn, id)
  }

  /// Create DB type : default implementation
  async fn create(conn: &'_ dgraph::Client, form: &'_ F) -> Result<i64, Error>{
    create_node::<F>(conn, form)
  }

  /// Update DB type : default implementation
  async fn update(conn: &'_ dgraph::Client, id: i64, form: &'_ F) -> Result<Self, Error> {
    update_node::<Self, F>(conn, id, form, Self::db_type_name())
  }
}


/**
 * Content that can be liked
 * 
 * @param F: form type for creating like
 */
#[async_trait]
pub trait CrudEdge<F>
where
  Self : Edge<F> + Sized {

  /// Like some content
  async fn create(conn: &dgraph::Client, form: &F) -> Result<Self, Error> {
    let edge = Self::from_form(form)?;
    let num_edges = create_edge::<Self, F>(conn, &edge);
    match num_edges {
      Ok(i) => Ok(edge),
      Err(e) => e
    }
  }

  /// Remove a like
  async fn delete(conn: &dgraph::Client, form: &F) -> Result<usize, Error> {
    // Create default edge, to avoid defining and implementing the Form trait
    let edge = Self::from_form(&form)?;
    delete_edges(conn,
      Some(edge.from()),
      Some(edge.to()),
      Self::db_type_name())
  }
}


// pub trait Followable<F, C> {

//   fn follow(conn: &C, form: &F) -> Result<Self>
//   where
//     Self: Sized;

//   fn ignore(conn: &C, form: &F) -> Result<usize>
//   where
//     Self: Sized;
// }

// pub trait Joinable<F, C> {

//   fn join(conn: &C, form: &F) -> Result<Self>
//   where
//     Self: Sized;

//   fn leave(conn: &C, form: &F) -> Result<usize>
//   where
//     Self: Sized;
// }

// /**
//  * Content that can be liked
//  * 
//  * @param F: form type for creating like
//  */
// #[async_trait]
// pub trait Likeable<F>
// where
//   Self : Edge<F> + Sized {

//   /// Get all likes
//   async fn read(conn: &dgraph::Client, content_id: i64) -> Result<Vec<Self>, Error> {
//     read_node::<Self>(conn, content_id, Self::db_type_name())
//   }

//   /// Like some content
//   async fn like(conn: &dgraph::Client, form: &F) -> Result<Self, Error> {
//     let edge = Self::from_form(form)?;
//     let num_edges = create_edge::<Self>(conn, &edge);
//     match num_edges {
//       Ok(i) => Ok(edge),
//       Err(e) => e
//     }
//   }

//   /// Remove a like
//   async fn remove(conn: &dgraph::Client, form: &F) -> Result<usize, Error> {
//     // Create default edge, to avoid defining and implementing the Form trait
//     let edge = Self::from_form(&form)?;
//     delete_edges(conn,
//       Some(edge.from()),
//       Some(edge.to()),
//       Self::db_type_name())
//   }
// }


// #[async_trait]
// pub trait Bannable<F>
// where
//   Self : Edge<F> + Sized {

//   /// Like some content
//   async fn ban(conn: &dgraph::Client, form: &F) -> Result<Self, Error> {
//     let edge = Self::from_form(form)?;
//     let num_edges = create_edge::<Self>(conn, &edge);
//     match num_edges {
//       Ok(i) => Ok(edge),
//       Err(e) => e
//     }
//   }

//   /// Remove a like
//   async fn unban(conn: &dgraph::Client, form: &F) -> Result<usize, Error> {
//     // Create default edge, to avoid defining and implementing the Form trait
//     let edge = Self::from_form(&form)?;
//     delete_edges(conn,
//       Some(edge.from()),
//       Some(edge.to()),
//       Self::db_type_name())
//   }
// }


// pub trait Saveable<F, C> {

//   fn save(conn: &C, form: &F) -> Result<Self>
//   where
//     Self: Sized;

//   fn unsave(conn: &C, form: &F) -> Result<usize>
//   where
//     Self: Sized;
// }

// pub trait Readable<F, C> {

//   fn mark_as_read(conn: &C, form: &F) -> Result<Self>
//   where
//     Self: Sized;

//   fn mark_as_unread(conn: &C, form: &F) -> Result<usize>
//   where
//     Self: Sized;
// }