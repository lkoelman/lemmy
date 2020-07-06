use crate::db::*;


/**
 * A type that is read from or stored in the database.
 */
pub trait Node {
  fn db_type_name() -> &'static str;
}


/**
 * Directional relationship between content
 * with an i32 id
 * 
 * TODO: in impl, add #[serde(skip)] to fields 'to' and 'from'
 */
pub trait Edge<F> {
  fn from(&self) -> &i64;
  fn to(&self) -> &i64;
  fn db_type_name() -> &'static str;
  fn from_form(form: &F) -> Result<Self, Error>;
}

/**
 * Read DB type : default implementation
 * NOTE: the 'de lifetime of the Deserialize trait is the lifetime 
 * of data that may be borrowed by Self when deserialized. 
 * See the page Understanding deserializer lifetimes 
 * for a more detailed explanation of these lifetimes.
 */
pub async fn read_node<T>(
    conn: &dgraph::Client,
    id: i64,
    type_name: &str,
  ) -> Result<T, Error> 
  where
    T : DeserializeOwned + Node {

  let type_name = T::db_type_name();

  let q = format!(r#"query {{
    {t_name}(func: uid({id})) @filter(type({t_name})) {{
    expand(type_name)
    }}}}"#, id=id, t_name=type_name);

  let mut txn = conn.new_read_only_txn();
  let resp = txn.query(q).await?;

  let res:T = resp.try_into_owned()?;
  Ok(res)
}


/**
 *  Read all nodes of type.
 * 
 * @param T : struct with field Vec<N> where N
 *            is the node type.
 */
pub async fn list_nodes<T>(
    conn: &dgraph::Client,
  ) -> Result<T, Error>
  where
    T : DeserializeOwned + Node {

  let type_name = T::db_type_name();

  let q = format!(r#"query {{
    all(func: type({type_name})) {{
      uid
      expand(type_name)
    }}}}"#, type_name=type_name);

  println!("GraphQL sent:\n{}", q);

  let mut txn = conn.new_read_only_txn();
  let resp = txn.query(q).await?;

  println!("JSON received:\n{}", std::str::from_utf8(&resp.json)?);

  let res: T = resp.try_into_owned()?;
  Ok(res)
}


/**
 * Delete node, and outgoing edges (both scalar predicates/fields,
 * and node-node edges)
 */
pub async fn delete_node(
    conn: &dgraph::Client,
    id: i64,
  ) -> Result<usize, Error> {

  // NOTE: The pattern S * * deletes all known edges out of a node 
  // (the node itself may remain as the target of edges), 
  //any reverse edges corresponding to the removed edges and 
  // any indexing for the removed data

  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();
  mu.set_delete_nquads(format!(r#"uid({}) * * ."#, id));

  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");

  Ok(resp.uids.len())
}

/**
 * Create DB type : default implementation
 * 
 * @return  uid : i64
 *          Unique ID of new node in database.
 */
pub async fn create_node<F>(
      conn: &dgraph::Client,
      form: &F,
  ) -> Result<i64, Error>
  where
      // T : Sized + DeserializeOwned,
      F : Sized + Serialize + Node {

  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();

  let type_name = F::db_type_name();
  
  // method A: serialize struct and add fields
  let dict = serde_json::to_value(form)?
              .as_object_mut()
              .expect("Could not convert to map.");
  
  // Add prefix "TypeName." to each field name
  for key in dict.keys() {
    dict.insert(format!("{}.{}", type_name, key),
                dict.remove(key).expect("key"));
  }
  dict.insert("uid".into(), "_:x".into());
  dict.insert("dgraph.type".into(), type_name.into());
  
  mu.set_set_json(dict).expect("JSON");

  // method B: serialize struct directly
  // mu.set_set_json(form).expect("JSON");

  // println!("JSON sent: {}", std::str::from_utf8(&mu.set_json)?);

  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");

  // println!("uids received:\n{:?}", resp.uids);
  // println!("JSON received:\n{}", std::str::from_utf8(&resp.json)?);

  if resp.uids.len() != 1 {
    failure::bail!("Failed to create node.")
  }

  let uid: i64 = resp.uids.values().next().expect("None").parse()?;
  // resp.uids.values().next().expect("None").parse::<i64>()

  Ok(uid)
}


/**
 * Create DB edge : default implementation
 * 
 * @return  uid : i64
 *          Unique ID of new node in database.
 */
pub async fn create_edge<T, F>(
    conn: &dgraph::Client,
    form: &T,
  ) -> Result<usize, Error>
  where
    T : Sized + Serialize + Edge<F> {


  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();

  let edge_def = format!(
    r#"uid({}) <{}> uid({})"#,
    form.from(), T::db_type_name(), form.to());


  let dict = serde_json::to_value(form)?
              .as_object_mut()
              .expect("Could not convert to map.");

  let facets_def = dict.iter().map(|a| format!("{}={:?}", a.0, a.1))
                              .collect::<Vec<_>>()
                              .join(",");

  let edge_facets_def = format!(
    r#"{} ({}) ."#, edge_def, facets_def
  );

  mu.set_set_nquads(edge_facets_def);
    
  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");
 
  if resp.txn?.preds.len() != 1 {
    failure::bail!("Failed to create edge.")
  }

  Ok(1)
}


/**
 * Delete node, and outgoing edges (both scalar predicates/fields,
 * and node-node edges)
 */
pub async fn delete_edges (
    conn: &dgraph::Client,
    from_id: Option<i64>,
    to_id: Option<i64>,
    predicate: Option<&str>,
  ) -> Result<usize, Error> {

  // NOTE: The pattern S * * deletes all known edges out of a node 
  // (the node itself may remain as the target of edges), 
  //any reverse edges corresponding to the removed edges and 
  // any indexing for the removed data

  let subject = match from_id {
    Some(i) => format!("uid({})", i),
    None => "*".into(),
  };

  let object = match to_id {
    Some(i) => format!("uid({})", i),
    None => "*".into(),
  };

  let pred = match predicate {
    Some(p) => format!("<{}>", p),
    None => "*".into(),
  };

  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();
  mu.set_delete_nquads(format!(
    r#"{} {} {} ."#,
    subject, pred, object));

  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");

  Ok(resp.uids.len())
}


/**
* Update DB node using form.
* 
* @param form : form containing subset of the fields in T
*/
pub async fn update_node<T: DeserializeOwned, F: Serialize>(
    conn: &dgraph::Client,
    id: i64,
    form: &F,
    type_name: &str,
  ) -> Result<T, Error> {

  let q = format!(r#"query {{
    {t_name}(func: uid({id})) @filter({t_name}) {{
      expand({t_name})
    }}}}"#, id=id, t_name=type_name);

  let mut mu = Mutation::new();
  mu.set_set_json(form).expect("JSON");

  let mut txn = conn.new_mutated_txn();
  let resp = txn.upsert(q, mu).await?;
  txn.commit().await?;

  let res: T = resp.try_into_owned()?;
  Ok(res)
}
