use bevy::prelude::Component;
use uuid::Uuid;

#[derive(Component)]
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
