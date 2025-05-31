use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CommitData {
    pub parent: String,
    pub message: String,
    pub timestamp: u64,
    pub files: HashMap<String, String>,
}