pub mod reference_image;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarImage {
    pub filepath: String,
    pub similarity: u32,
}
