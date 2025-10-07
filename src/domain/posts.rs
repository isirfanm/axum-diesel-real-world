use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct PostModel {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug)]
pub enum PostError {
    InternalServerError,
    NotFound(Uuid),
    InfraError(InfraError),
}
