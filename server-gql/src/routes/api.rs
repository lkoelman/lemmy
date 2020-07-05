use super::*;
use crate::api::{Query, Mutation, Schema, Context};
use crate::rate_limit::RateLimit;

use actix_web;
use actix_web::{web, Error, HttpResponse, HttpRequest};
// use juniper::http::graphiql::graphiql_source;
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use dgraph_tonic::Client;

// TODO: follow official example in github/graphq-rust/juniper/juniper_actix/examples
//       using async/await and not the example by husseinraoouf using Futures
// - async/await : see https://github.com/graphql-rust/juniper/blob/master/juniper_actix/examples/actix_server.rs

/// Handler for GET requests at GraphQL endpoint
// TODO: graphiql endpoint (see juniper_actix example)

// TODO: for usage see :
// - https://github.com/graphql-rust/juniper/blob/master/juniper_actix/examples/actix_server.rs
// - https://github.com/graphql-rust/juniper/blob/master/juniper_actix/src/lib.rs
// - NOTE: resolution with context implemented in QueryField/MutationField, not here
/// Handler POST requests at GraphQL endpoint
async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    // Actix extractors, order doesn't matter
    schema: web::Data<Arc<Schema>>,
    db_client: web::Data<Client>
) -> Result<HttpResponse, Error> {
    // context = client
    graphql_handler(&schema, &db_client, req, payload).await
}

// async fn graphql(
//     schema: web::Data<Arc<Schema>>,
//     data: web::Json<GraphQLRequest>,
//     db_pool: web::Data<DbPool>,
// ) -> impl Result<HttpResponse, Error> {
    
//     let ctx = Context {
//         db_con: db_pool.get().unwrap(),
//     };

//     let user = web::block(move || {
//         let res = data.execute(&schema, &ctx);
//         Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
//     })
//     .await?;

//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .body(user))
// }


pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {

    // Root schema based on the graphql root Query and Mutation objects
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    cfg.service(
        web::scope("/api/v1")
            .data(schema.clone())
            // TODO: .data(db_client) ?? make only one db client
            .service(web::resource("/graphql")
                .route(web::post().to(graphql)))
            // .service(web::resource("/graphiql")
            //     .route(web::get().to(graphiql)))
      
    );
}