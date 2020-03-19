#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate validator;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;
extern crate juniper_codegen;
extern crate validator_derive;

mod error;
mod graphql;
mod schema;
mod web;

fn main() {
    web::launch();
}
