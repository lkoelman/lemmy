/**
 * GraphQL request handlers for actix framework
 * 
 * Based on following examples:
 * https://github.com/graphql-rust/juniper/blob/master/juniper_actix/examples/actix_server.rs
 * https://github.com/actix/examples/blob/master/juniper/src/main.rs
 * https://github.com/actix/examples/blob/master/juniper-advanced/src/handlers.rs
 * 
 */

use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::http::GraphQLRequest;
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};

use crate::api::schema::{Schema, create_schema, Context};

/**
 * Handle POST/GET requests to the GraphiQL endpoint.
 * 
 * This is a web IDE for editing and testing GraphQL queries and mutations.
 * 
 */
pub async fn graphiql_handler() -> Result<HttpResponse, Error> {
    gqli_handler("/", None).await
}

/**
 * Handle POST/GET requets for the GraphQL Playground endpoint.
 * 
 * This is web IDE for editing and testing GraphQL queries and mutations.
 */
pub async fn playground_handler() -> Result<HttpResponse, Error> {
    play_handler("/", None).await
}

// Uaing new wrapper function
// async fn graphql(
//     req: actix_web::HttpRequest,
//     payload: actix_web::web::Payload,
//     schema: web::Data<Schema>,
// ) -> Result<HttpResponse, Error> {
//     let context = Database::new();
//     graphql_handler(&schema, &context, req, payload).await
// }

/**
 * Handle POST and GET requests at GraphQL endpoint.
 */
pub async fn graphql(
    conn: web::Data<dgraph_tonic::Client>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: conn.to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}