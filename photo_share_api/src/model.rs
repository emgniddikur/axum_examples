use async_graphql::{EmptySubscription, Enum, InputObject, Object, Schema, SimpleObject};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
}

impl Default for PhotoCategory {
    fn default() -> Self {
        PhotoCategory::Portrait
    }
}

#[derive(Clone, SimpleObject)]
struct Photo {
    id: usize,
    name: String,
    description: String,
    category: PhotoCategory,
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

#[derive(InputObject)]
struct PostPhotoInput {
    name: String,
    description: String,
    #[graphql(default_with = "PhotoCategory::default()")]
    category: PhotoCategory,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_photo(&self, input: PostPhotoInput) -> Photo {
        let mut id = SEQUENCE_ID.lock().unwrap();

        let photo = Photo {
            id: *id,
            name: input.name,
            description: input.description,
            category: input.category,
        };

        PHOTOS.lock().unwrap().push(photo.clone());

        *id += 1;

        photo
    }
}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;
