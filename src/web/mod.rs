//
// web.rs
//
pub mod cors;

use std::sync::{Arc, RwLock};

use crate::graphql::{Ctx, Mutation, Query, Schema};
use rocket::response::content;
use rocket::State;

#[get("/graphql/explorer")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[options("/graphql")]
fn post_graphql_cors_handler() -> content::Plain<String> {
    content::Plain("".to_string())
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    ctx: State<Ctx>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, ctx.inner())
}

pub fn launch() {
    rocket::ignite()
        .manage(Ctx {
            todos: Arc::new(RwLock::new(vec![])),
        })
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, post_graphql_cors_handler],
        )
        .attach(cors::CORS())
        .launch();
}
