// use crate::api::{Query, Mutation, Schema, Context};
use crate::rate_limit::RateLimit;

use actix_web;
use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::api::handlers::{graphiql_handler, graphql, playground_handler};
use crate::api::schema::create_schema;

/**
 * Register GraphQL handlers.
 */
pub fn register(cfg: &mut web::ServiceConfig) {
  // Root schema based on the graphql root Query and Mutation objects
  let schema = std::sync::Arc::new(create_schema());

  cfg.service(
    web::scope("/api/v1")
      .data(schema.clone())
      .service(
        web::resource("/")
          .route(web::post().to(graphql))
          .route(web::get().to(graphql)),
      )
      .service(web::resource("/playground").route(web::get().to(playground_handler)))
      .service(web::resource("/graphiql").route(web::get().to(graphiql_handler))),
  );
}
