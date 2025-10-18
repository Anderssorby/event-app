
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPerson {
    name: String,
}

