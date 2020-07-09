use std::collections::HashMap;
use std::string::ToString;
use dgraph_tonic::{Response as DgraphResponse, DgraphError};
use indextree::{Arena as Tree};
use crate::db::*;

/**
 * A type that is read from or stored in the database.
 */
pub trait Node {
  fn db_type_name() -> &'static str;
  fn set_id(&mut self, id: i64);
  fn get_id(&self) -> &i64;
}

/**
 * Directional relationship between content
 * with an i32 id
 *
 * TODO: in impl, add #[serde(skip)] to fields 'to' and 'from'
 */
pub trait Edge {
  fn from(&self) -> i64;
  fn to(&self) -> i64;
  fn db_type_name() -> &'static str;
  // fn from_form(form: &F) -> Result<Self, Error>;
}

/// Result type for retrieving nodes (for deserialization)
#[derive(Debug, Deserialize)]
pub struct NodeList<T> {
  pub all: Vec<T>,
}

/**
 * Functions allowed in Dgraph queries.
 * 
 * See https://dgraph.io/docs/query-language/#functions
 */
#[allow(non_camel_case_types)]
#[derive(Debug, Display, Eq, PartialEq)]
pub enum DgraphFunction {
  NOP, // no func (empty)
  has(String),
  uid(i64),
  #[strum(to_string = "uid_in")]
  uid_link(i64),
  #[strum(to_string = "type")]
  type_name(String),
  eq(String),
  ge(String),
  gt(String),
  le(String),
  lt(String),
  allofterms(String),
  anyofterms(String),
  regexp(String),
  fuzzy_match(String),
  alloftext(String),
  // other functions
  func(String, String), // name, args
  // connecting filters
  AND, OR, NOT,
  #[strum(to_string = "(")]
  OPEN_PAR,
  #[strum(to_string = ")")]
  CLOSE_PAR,
  // geo location
  // near, within, intersects, 
}

type DFn = DgraphFunction;

impl DgraphFunction {
  /// Print function for use in Dgraph query
  pub fn print(&self) -> String {
    match self {
      // TODO: implement me
      DFn::func(name, args) => format!("{}({})", name, args),
      DFn::has(id) => format!("{}({})", self, id),
      _ => "ERROR".to_string(),
    }
  }
}


/**
 * Helper for bulding Dgraph queries.
 */
pub struct QueryBuilder<'a> {
  client: &'a dgraph_tonic::Client,
  root_func: DgraphFunction,
  nodes: Vec<HashMap<String, String>>,
  tree: Tree::new(),
  last_filter: DgraphFunction,
}


// TODO: support parentheses in filters

/**
 * Builder pattern for Dgraph queries.
 * 
 * EXAMPLE
 * -------
 */
impl<'a> QueryBuilder<'a> {
  
  /**
   * @param name : (optional) a name for the query
   */
  pub fn new(client: &dgraph_tonic::Client, name: Option<String>) -> QueryBuilder {

    let node = Self::init_node_block(name.unwrap_or("all".into()));

    QueryBuilder {
      client: client,
      root_func: DgraphFunction::uid(0),
      nodes: vec![node],
      last_filter: DgraphFunction::NOP,
    }

  }

  pub fn init_node_block(name: String) -> HashMap<String, String> {
    let mut nested = HashMap::new();
    nested.insert("_:pred_name".into(), name);
    // nested.insert("_:sort".into(), "".into());
    // nested.insert("_:filter".into(), "".into());
    nested
  }

  //===========================================================================
  // Builder pattern methods (for chaining)

  /**
   * Set the root query function.
   */
  pub fn root_query(&'a mut self, func: DgraphFunction) -> &'a mut QueryBuilder {
    self.root_func = func;
    self
  }

  /**
   * Query scalar predicate (at current level of nesting/
   * linked predicates)
   * 
   * @param pred: name of the predicate or function
   *              E.g. "predicate name", "expand(_all_)", 
   *              "expand(type_name)", "count(predicate_name)"
   */
  pub fn scalar_predicate(
      &'a mut self,
      pred: String,
      suffix: Option<String>
    ) -> &'a mut QueryBuilder
  {
    let i_node = self.nodes.len() - 1;
    if !self.nodes[i_node].contains_key(&pred) {
      self.nodes[i_node].insert(pred, suffix.unwrap_or("".to_string()));
    }
    self
  }

  /**
   * Query linked predicate (i.e. follow edge)
   */
  pub fn linked_predicate(&'a mut self, pred: String) -> &'a mut QueryBuilder {
    let nested = Self::init_node_block(pred);
    self.nodes.push(nested);
    self
  }


  /**
   * Add filter function to current predicate or root query.
   */
  pub fn filter(&'a mut self, func: DgraphFunction) -> &'a mut QueryBuilder {

    let suffix = match self.last_filter {
      DFn::AND | DFn::OR | DFn::NOT => format!("{} {}",
          self.last_filter, func.print()),
      _ => func.print(),
    }
    self.last_filter = func;

    self.nodes[self.nodes.len()-1].entry("_:filter".to_string())
      .and_modify(|s| s.push_str(&format!(", {}", suffix)))
      .or_insert(suffix);

    self
  }

  /**
   * Add sorting to current predicate or root query.
   * 
   * @param order : "asc" or "desc"
   */
  pub fn sort(&'a mut self, pred: String, order: &str) -> &'a mut QueryBuilder {

    assert!(["asc", "desc"].contains(&order));
    let clause = format!("order{}: {}", order, pred);

    self.nodes[self.nodes.len()-1].entry("_:sort".to_string())
      .and_modify(|s| s.push_str(&format!(", {}", clause)))
      .or_insert(clause);

    self
  }

  /**
   * Execute the query in Dgraph database
   */
  pub async fn execute(&self) -> Result<DgraphResponse, DgraphError> {
    let q = self.build();
    let mut txn = self.client.new_read_only_txn();
    txn.query(q).await
  }

  //===========================================================================
  // Utility methods

  /**
   * Build string representation of the query
   */
  pub fn build(&self) -> String {
    format!(r#"query {{ {node} }}"#, node=self.build_node(0))
  }

  /***
   * Build query block for graph node at nesting level i.
   */
  pub fn build_node(&self, i: usize) -> String {

    if i >= self.nodes.len() {
      return "".to_string()
    }

    let scalar_preds: String = self.nodes[i].iter()
      .filter(|(k, v)| !k.starts_with("_:"))
      .map(|(k, v)| [*k, *v].concat())
      .collect::<Vec<String>>()
      .join(",\n");

    // sort clauses occur in argument list to query or predicate
    let sort_clause: &String = self.nodes[i]
      .get("_:sort").unwrap_or(&"".to_owned());

    // filter clauses are wrapped in @filter(...)
    let filter_clause = match self.nodes[i].get("_:filter") {
      Some(clause) => format!("@filter({})", &clause),
      None => "".to_string(),
    };


    format!(r#"{node_name}({root_query} {sort_clause}) {filter_clause} {{
        {scalar_preds}
        {nested_preds}
      }}}}"#,
      node_name = self.nodes[i]["_:pred_name"],
      root_query = if i==0 { self.root_func.print() + ", " } else { "".to_string() },
      sort_clause = sort_clause,
      filter_clause = filter_clause,
      scalar_preds = scalar_preds,
      nested_preds = self.build_node(i+1),
    )
  }

  
}

/**
 * Read DB type : default implementation
 * NOTE: the 'de lifetime of the Deserialize trait is the lifetime
 * of data that may be borrowed by Self when deserialized.
 * See the page Understanding deserializer lifetimes
 * for a more detailed explanation of these lifetimes.
 */
pub async fn read_node<T>(conn: &dgraph::Client, id: i64) -> Result<T, Error>
where
  T: Node + DeserializeOwned,
{
  let type_name = T::db_type_name();

  let q = format!(
    r#"query {{
    {t_name}(func: uid({id})) @filter(type({t_name})) {{
    expand(type_name)
    }}}}"#,
    id = id,
    t_name = type_name
  );

  let mut txn = conn.new_read_only_txn();
  let resp = txn.query(q).await?;

  let res: T = resp.try_into_owned()?;
  Ok(res)
}

/**
 *  Find nodes by attribute (predicate) value.
 *
 * @param T : struct with field Vec<N> where N
 *            is the node type.
 *
 * @param val : text representation of value
 *              e.g. result of format!("{:?}", val)
 */
pub async fn find_nodes<T>(conn: &dgraph::Client, pred: &str, val: &str) -> Result<Vec<T>, Error>
where
  T: DeserializeOwned + Node,
{
  let type_name = T::db_type_name();

  let q = format!(
    r#"query {{
    nodeList(func: eq({pred}, {val})) @filter(type({type_name})) {{
      uid
      expand(type_name)
    }}}}"#,
    type_name = type_name,
    pred = pred,
    val = val
  );

  let mut txn = conn.new_read_only_txn();
  let resp = txn.query(q).await?;

  let res: NodeList<T> = resp.try_into_owned()?;
  Ok(res.all)
}

/**
 * Find a single node by predicate value.
 */
pub async fn find_node<T>(
    conn: &dgraph::Client,
    pred: &str,
    val: &str,
  ) -> Result<T, Error>
  where
    T: DeserializeOwned + Node {

  let nodes = find_nodes::<T>(conn, pred, val).await?;

  if nodes.len() != 1 {
    failure::bail!(format!("Expected 1 node but found {}", nodes.len()));
  }
  Ok(nodes[0])
}

/**
 *  Read all nodes of type.
 *
 * @param T : struct with field Vec<N> where N
 *            is the node type.
 */
pub async fn list_nodes<T>(conn: &dgraph::Client) -> Result<Vec<T>, Error>
where
  T: DeserializeOwned + Node,
{
  let type_name = T::db_type_name();

  let q = format!(
    r#"query {{
    nodeList(func: type({type_name})) {{
      uid
      expand(type_name)
    }}}}"#,
    type_name = type_name
  );

  // println!("GraphQL sent:\n{}", q);

  let mut txn = conn.new_read_only_txn();
  let resp = txn.query(q).await?;

  // println!("JSON received:\n{}", std::str::from_utf8(&resp.json)?);

  let res: NodeList<T> = resp.try_into_owned()?;
  Ok(res.all)
}

/**
 * Delete node, and outgoing edges (both scalar predicates/fields,
 * and node-node edges)
 */
pub async fn delete_node(conn: &dgraph::Client, id: i64) -> Result<usize, Error> {
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
 * @param   node : T impl Node
 *          Node definition. Field 'id' will be set
 *          if operation succesful.
 *
 * @return  uid : i64
 *          Unique ID of new node in database.
 */
pub async fn create_node<T>(conn: &dgraph::Client, node: &mut T) -> Result<i64, Error>
where
  T: Node + Sized + Serialize,
{
  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();

  let type_name = T::db_type_name();

  // method A: serialize struct and add fields
  let dict = serde_json::to_value(node)?
    .as_object_mut()
    .expect("Could not convert to map.");

  // TODO: handle links to other nodes (with uid)
  // - see https://dgraph.io/docs/mutations/#edges-between-nodes
  // - in RDF/nquad format this is "<0x111> <link> <0x222>"
  // - in JSON format this is a nested value : "link" : { "uid": "0x222" }
  // SOLUTIONS
  // 1. on linked uid fields, use #[serde(rename(serialize = "link.field_name"))]
  //    - handle these fields here when iterating over the dict
  //      if key.startswith("link.") => remove key => insert dict

  // Add prefix "TypeName." to each field name
  for key in dict.keys() {
    dict.insert(
      format!("{}.{}", type_name, key),
      dict.remove(key).expect("key"),
    );
  }
  dict.insert("uid".into(), "_:x".into());
  dict.insert("dgraph.type".into(), type_name.into());
  mu.set_set_json(dict).expect("JSON");

  // method B: serialize struct directly
  // mu.set_set_json(form).expect("JSON");

  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");

  if resp.uids.len() != 1 {
    failure::bail!("Failed to create node.")
  }

  let uid: i64 = resp.uids.values().next().expect("None").parse()?;
  // resp.uids.values().next().expect("None").parse::<i64>()

  node.set_id(uid);
  Ok(uid)
}

/**
* Update DB node using form.
*
* @param form : form containing subset of the fields in T
* @return node : the modified node after update
*/
pub async fn update_node_fields<T, F>(
  conn: &dgraph::Client,
  id: i64,
  form: &F,
  type_name: &str,
) -> Result<T, Error>
where
  T: DeserializeOwned,
  F: Serialize,
{
  let q = format!(
    r#"query {{
    {t_name}(func: uid({id})) @filter({t_name}) {{
      expand({t_name})
    }}}}"#,
    id = id,
    t_name = type_name
  );

  let mut mu = Mutation::new();
  mu.set_set_json(form).expect("JSON");

  let mut txn = conn.new_mutated_txn();
  let resp = txn.upsert(q, mu).await?;
  txn.commit().await?;

  let res: T = resp.try_into_owned()?;
  Ok(res)
}

/**
* Update DB node using full representation.
*
* @param form : form containing subset of the fields in T
* @return updated : the modified node after update
*/
pub async fn update_node<T>(
  conn: &dgraph::Client,
  updated: &T
) -> Result<T, Error>
where
  T: Node + DeserializeOwned + Serialize,
{

  update_node_fields::<T, T>(
    conn,
    *updated.get_id(),
    updated,
    T::db_type_name()).await
}

/**
* Update DB node using form.
*
* @param form : form containing subset of the fields in T
* @return node : the modified node after update
*
* EXAMPLE
* -------
* ```
* use serde_json::{Value, Map, Number};
* 
* let mut inner_map = Map::new();
* inner_map.insert("x".to_string(), Value::Number(Number::from(10u64)));
* inner_map.insert("y".to_string(), Value::Number(Number::from(20u64)));
* 
* let mut map = Map::new();
* map.insert("key1".to_string(), Value::String("test".to_string()));
* map.insert("key2".to_string(), Value::Array(vec![
*         Value::String("a".to_string()),
*         Value::String("b".to_string())]));
* map.insert("key3".to_string(), Value::Object(inner_map));
* 
* update_node_dict<CompositeType>(conn, id, Value::Object(map))
* ```
*/
pub async fn update_node_dict<T>(
  conn: &dgraph::Client,
  id: i64,
  dict: &serde_json::Value,
) -> Result<T, Error>
where
  T: Node + DeserializeOwned,
{
  let q = format!(
    r#"query {{
    {t_name}(func: uid({id})) @filter({t_name}) {{
      expand({t_name})
    }}}}"#,
    id = id,
    t_name = T::db_type_name()
  );

  let mut mu = Mutation::new();
  mu.set_set_json(dict).expect("JSON");

  let mut txn = conn.new_mutated_txn();
  let resp = txn.upsert(q, mu).await?;
  txn.commit().await?;

  let res: T = resp.try_into_owned()?;
  Ok(res)
}

/**
 * Create DB edge : default implementation
 *
 * @return  uid : i64
 *          Unique ID of new node in database.
 */
pub async fn create_edge<T, F>(conn: &dgraph::Client, form: &T) -> Result<usize, Error>
where
  T: Sized + Serialize + Edge,
{
  let mut txn = conn.new_mutated_txn();
  let mut mu = Mutation::new();

  let edge_def = format!(
    r#"uid({}) <{}> uid({})"#,
    form.from(),
    T::db_type_name(),
    form.to()
  );

  let dict = serde_json::to_value(form)?
    .as_object_mut()
    .expect("Could not convert to map.");

  let facets_def = dict
    .iter()
    .map(|a| format!("{}={:?}", a.0, a.1))
    .collect::<Vec<_>>()
    .join(",");

  let edge_facets_def = format!(r#"{} ({}) ."#, edge_def, facets_def);

  mu.set_set_nquads(edge_facets_def);
  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");
  if resp.txn.expect("Failed.").preds.len() != 1 {
    failure::bail!("Failed to create edge.")
  }

  Ok(1)
}

/**
 * Delete node, and outgoing edges (both scalar predicates/fields,
 * and node-node edges)
 */
pub async fn delete_edges(
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
  mu.set_delete_nquads(format!(r#"{} {} {} ."#, subject, pred, object));

  let resp = txn.mutate(mu).await?;
  txn.commit().await?; // .expect("Transaction is commited");

  Ok(resp.uids.len())
}
