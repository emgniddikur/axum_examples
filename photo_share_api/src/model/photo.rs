use async_graphql::{Enum, SimpleObject};

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum PhotoCategory {
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
pub struct Photo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub category: PhotoCategory,
}
