use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Sample {
    pub id: Uuid,
}

impl Sample {
    pub fn new(id: Uuid) -> Self {
        Self { id: id }
    }
}
