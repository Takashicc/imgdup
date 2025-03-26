pub mod reference_image;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimilarImage {
    pub filepath: String,
    pub similarity: u32,
    pub is_deleted: bool,
    pub error_message: Option<String>,
}
