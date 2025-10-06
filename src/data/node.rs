use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub id: Uuid,
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub parent_id: Option<Uuid>,
}
