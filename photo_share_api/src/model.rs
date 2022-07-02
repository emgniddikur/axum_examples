use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        42
    }
}

pub type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;
