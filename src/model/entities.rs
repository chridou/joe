use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    id: Uuid,
    username: String,
    surname: String,
    last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Customer {
    id: Uuid,
    name: String,
    address_line_1: String,
    address_line_2: Option<String>,
    address_zip: Option<String>,
    address_city: Option<String>,
    address_country: Option<String>,
}
