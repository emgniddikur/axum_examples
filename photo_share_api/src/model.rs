use async_graphql::{EmptySubscription, Object, Schema, SimpleObject};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone, SimpleObject)]
struct Photo {
    id: usize,
    name: String,
    description: String,
}

static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(1));
static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }

    async fn all_photos(&self) -> Vec<Photo> {
        PHOTOS.lock().unwrap().clone()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_photo(&self, name: String, description: String) -> Photo {
        let mut id = SEQUENCE_ID.lock().unwrap();

        let photo = Photo {
            id: *id,
            name,
            description,
        };

        PHOTOS.lock().unwrap().push(photo.clone());

        *id += 1;

        photo
    }
}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;
