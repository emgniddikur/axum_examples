use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn total_todos(&self) -> usize {
        1
    }
}

pub type TodoSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
