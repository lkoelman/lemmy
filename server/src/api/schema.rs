/**
 * GraphQL public schema for client queries/mutations.
 * 
 * Based on following examples:
 * (async Queries/Mutations) https://github.com/graphql-rust/juniper/blob/master/examples/warp_subscriptions/src/main.rs
 * https://github.com/graphql-rust/juniper/blob/master/juniper/src/tests/schema.rs
 * https://github.com/actix/examples/blob/master/juniper/src/schema.rs
 * https://github.com/actix/examples/tree/master/juniper-advanced/src/schemas
 * 
 */

use juniper::{
    RootNode, graphql_object, FieldResult,
    EmptyMutation, EmptySubscription,
};
use dgraph_tonic::{Client};


// TODO: dispatch queries to impl Oper in api/ops/...
// - for dispatching, see routes/api.rs and the Juniper examples
// - dispatching happens in the root Query/Mutation and field 
//   implementations of GraphQL objects

/**
 * Context for executing GraphQL queries.
 */
pub struct Context {
    pub conn: Client,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

// TODO: dispatch root query ops
#[juniper::graphql_object(Context = Context)]
impl QueryRoot {

    #[graphql(description = "List of all users")]
    async fn getComments(context: &Context, ) -> Vec<Comment> {
        let conn: dgraph_tonic::Client = context.conn;

        
        Ok(comments)
    }
}

pub struct MutationRoot;



#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    // TODO: dispatch root mutation ops
}

// Export the schema
// pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

// pub fn create_schema() -> Schema {
//     Schema::new(QueryRoot, MutationRoot)
// }

// TODO: put <Context> in genertics for QueryRoot/MutationRoot?
pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}