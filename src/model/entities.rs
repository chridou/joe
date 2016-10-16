use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    id: Uuid,
    name: String,
}
