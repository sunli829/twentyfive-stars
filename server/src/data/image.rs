use uuid::Uuid;

#[async_graphql::InputObject]
#[derive(Debug, Clone)]
pub struct ImageInput {
    pub original_url: String,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub id: i32,
    pub node_id: Uuid,
    pub original_url: String,
}

#[async_graphql::Object]
impl Image {
    pub async fn id(&self) -> async_graphql::ID {
        self.node_id.into()
    }

    pub async fn original_url(&self) -> &str {
        &self.original_url
    }
}
