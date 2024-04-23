use models::product::Model;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
    pub struct Error {
    pub error: String
}


#[derive(Debug, Deserialize, Serialize)]
    pub struct ProductsResponse {
    pub products: Vec<Model>,
    pub pages: u64,
}


