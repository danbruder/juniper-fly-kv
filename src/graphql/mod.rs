//
// graphql.rs
//
use std::sync::{Arc, RwLock};

use juniper::FieldResult;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub struct Query;
pub struct Mutation;

pub struct Ctx {
    pub todos: Arc<RwLock<Vec<Todo>>>,
}

#[derive(Clone, GraphQLObject, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Todo {
    title: String,
}

graphql_object!(Query: Ctx |&self| {
    field todos(&executor) -> FieldResult<Vec<Todo>> {
        let val = executor.context().todos.read()?.clone();

        Ok(val)
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field insertTodo(&executor, input: String) -> FieldResult<Todo> {
        let mut todos = executor.context().todos.write().unwrap();

        let new_todo = Todo {
            title: input
        };
        todos.push(new_todo.clone());

        Ok(new_todo)
    }
});
