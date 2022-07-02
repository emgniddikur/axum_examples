use async_graphql::{EmptySubscription, Object, Schema};
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Photo {
    name: String,
    description: String,
}

static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_photo(&self, name: String, description: String) -> bool {
        let photo = Photo { name, description };
        PHOTOS.lock().unwrap().push(photo);

        true
    }
}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;
