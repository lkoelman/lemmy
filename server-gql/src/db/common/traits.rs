use super::dgraph_utils::*;
use crate::db::*;

// TODO: cannot use efficient async traits in current Rust.
// -> check situation: https://rust-lang.github.io/async-book/07_workarounds/06_async_in_traits.html

// /**
//  * Generic CRUD operation (create, read, update, delete)
//  * for any DB node type.
//  */
// #[async_trait]
// pub trait CrudNode<F>
// where
//   Self: Node + From<F> + Sized + DeserializeOwned,
//   F: Node + Sized + Serialize,
// {
//   /// Read DB type : default implementation
//   async fn read(conn: &'_ dgraph::Client, id: i64) -> Result<Self, Error> {
//     read_node::<Self>(conn, id, Self::db_type_name()).await
//   }

//   /// Delete DB type : default implementation
//   async fn delete(conn: &'_ dgraph::Client, id: i64) -> Result<usize, Error> {
//     delete_node(conn, id).await
//   }

//   /// Create DB type : default implementation
//   async fn create(conn: &'_ dgraph::Client, form: &'_ F) -> Result<Self, Error> {
//     // TODO: copy approach from CrudEdge::create, declare Node::from_form
//     // with default conversion
//     let node: Self = form.into();
//     let uid = create_node::<Self>(conn, &mut node).await?;
//     node
//   }

//   /// Update DB type : default implementation
//   async fn update(conn: &'_ dgraph::Client, id: i64, form: &'_ F) -> Result<Self, Error> {
//     update_node::<Self, F>(conn, id, form, Self::db_type_name()).await
//   }
// }


// /**
//  * Generic CRUD operations for any DB edge (non-scalar predicate) type.
//  *
//  * @param F: form type for creating edge
//  */
// #[async_trait]
// pub trait CrudEdge<F>
// where
//   Self: Edge + From<F> + Serialize + Sized,
// {
//   /// Like some content
//   async fn create(conn: &dgraph::Client, form: &F) -> Result<Self, Error> {
//     let edge: Self = form.into();
//     let num_edges = create_edge::<Self, F>(conn, &edge).await;
//     match num_edges {
//       Ok(i) => Ok(edge),
//       Err(e) => Err(e),
//     }
//   }

//   /// Remove a like
//   async fn delete(conn: &dgraph::Client, form: &F) -> Result<usize, Error> {
//     // Create default edge, to avoid defining and implementing the Form trait
//     let edge: Self = form.into();
//     delete_edges(
//       conn,
//       Some(edge.from()),
//       Some(edge.to()),
//       Some(Self::db_type_name()),
//     )
//     .await
//   }
// }
