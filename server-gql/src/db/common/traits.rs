use self::dgraph_utils::*;

/**
 * Generic CRUD operation (create, read, update, delete)
 * for any DB type.
 */
#[async_trait]
pub trait Crud<T, F> 
where
  T : Sized + DeserializeOwned,
  F : Sized + Serialize {

  /// Get the DB type
  fn db_type_name() -> &'static str;

  /// Read DB type : default implementation
  async fn read(conn: &'_ dgraph::Client, id: i32) -> Result<Self, Error> {
    read_node::<T>(conn, id, Self::db_type_name())
  }

  /// Delete DB type : default implementation
  async fn delete(conn: &'_ dgraph::Client, id: i32) -> Result<usize, Error> {
    delete_node(conn, id)
  }

  /// Create DB type : default implementation
  async fn create(conn: &'_ dgraph::Client, form: &'_ F) -> Result<i64, Error>{
    create_node::<F>(conn, form)
  }

  /// Update DB type : default implementation
  async fn update(conn: &'_ dgraph::Client, id: i32, form: &'_ F) -> Result<Self, Error> {
    update_node::<T, F>(conn, id, form, Self::db_type_name())
  }
}


pub trait Followable<F, C> {

  fn follow(conn: &C, form: &F) -> Result<Self>
  where
    Self: Sized;

  fn ignore(conn: &C, form: &F) -> Result<usize>
  where
    Self: Sized;
}

pub trait Joinable<F, C> {

  fn join(conn: &C, form: &F) -> Result<Self>
  where
    Self: Sized;

  fn leave(conn: &C, form: &F) -> Result<usize>
  where
    Self: Sized;
}

/**
 * Directional relationship between content
 * with an i32 id
 * 
 * TODO: add attributes #[serde(skip)] to fields 'to' and 'from'
 */
pub trait EdgeForm {
  fn from(&self) -> i32;
  fn to(&self) -> i32;
  fn score(&self) -> i16;
}

/**
 * Content that can be liked
 * 
 * @param F: form type for creating like
 */
#[async_trait]
pub trait Likeable<F: Like> {

  fn db_type_name() -> &'static str;

  /// Get all likes
  async fn read(conn: &dgraph::Client, content_id: i32) -> Result<Vec<Self>>
  where Self: Sized {
    // TODO: use read_edge here
    read_node::<T>(conn, id, Self::db_type_name())
  }

  /// Like some content
  async fn like(conn: &dgraph::Client, form: &F) -> Result<Self>
  where Self: Sized {

    // use Facets https://dgraph.io/docs/query-language/#facets-edge-attributes

    let mut mu = Mutation::new();
    mu.set_set_nquads(format!(
      r#"uid({}) <{}> uid({}) (score={}) ."#,
      form.from(), Self::db_type_name(), form.to(), form.score())
    );

    let mut txn = conn.new_mutated_txn();
    let resp = txn.mutate(mu).await?;
    txn.commit().await?; // .expect("Transaction is commited");

    resp.try_into::<Self>()?
  }

  /// Remove a like
  async fn remove(conn: &dgraph::Client, form: &F) -> Result<usize>
  where Self: Sized {

    let mut txn = conn.new_mutated_txn();
    let mut mu = Mutation::new();
    mu.set_delete_nquads(format!(
      r#"uid({user}) <{predicate_name}> uid({comment}) ."#,
      user=form.from(),
      predicate_name=Self::db_type_name(),
      comment=form.to())
    );

    let resp = txn.mutate(mu).await?;
    txn.commit().await?; // .expect("Transaction is commited");

    Ok(resp.uids.len())
  }
}


pub trait Bannable<F, C> {

  fn ban(conn: &C, form: &F) -> Result<Self>
  where
    Self: Sized;

  fn unban(conn: &C, form: &F) -> Result<usize>
  where
    Self: Sized;
}

pub trait Saveable<F, C> {

  fn save(conn: &C, form: &F) -> Result<Self>
  where
    Self: Sized;

  fn unsave(conn: &C, form: &F) -> Result<usize>
  where
    Self: Sized;
}

pub trait Readable<F, C> {

  fn mark_as_read(conn: &C, form: &F) -> Result<Self>
  where
    Self: Sized;

  fn mark_as_unread(conn: &C, form: &F) -> Result<usize>
  where
    Self: Sized;
}