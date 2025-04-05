use anyhow::{Context, Result};
use img_hash::{HasherConfig, ImageHash};

pub fn compute_hash(path: &str) -> Result<ImageHash> {
    let img = image::open(path).context("Failed to open image")?;
    let hasher = HasherConfig::new().hash_size(8, 8).to_hasher();
    Ok(hasher.hash_image(&img))
}

pub fn calculate_similarity(h1: &ImageHash, h2: &ImageHash) -> u32 {
    let dist = h1.dist(h2);
    let similarity = 100 - dist;
    similarity
}
