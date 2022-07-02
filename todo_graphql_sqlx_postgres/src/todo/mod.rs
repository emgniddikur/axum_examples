use async_graphql::{EmptySubscription, Object, Schema};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize)]
struct Todo {
    // id: Uuid,
    text: String,
}

static TODOS: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn total_todos(&self) -> usize {
        TODOS.lock().unwrap().len()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_todo(&self, text: String) -> bool {
        let todo = Todo { text };
        TODOS.lock().unwrap().push(todo);

        true
    }
}

pub type TodoSchema = Schema<QueryRoot, Mutation, EmptySubscription>;
