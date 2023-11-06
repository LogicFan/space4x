use bevy::prelude::Component;
use uuid::Uuid;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new_random() -> Self {
        ObjectId(Uuid::new_v4())
    }
}

impl ToString for ObjectId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
