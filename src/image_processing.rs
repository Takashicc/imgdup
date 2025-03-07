use anyhow::{Context, Result};
use dioxus::logger::tracing::debug;
use img_hash::{HasherConfig, ImageHash};

// TODO what is 8, 8?
// TODO is it using phash algorithm?
pub fn compute_hash(path: &str) -> Result<ImageHash> {
    debug!("Computing hash for: {:?}", path);
    let img = image::open(path).context("Failed to open image")?;
    let hasher = HasherConfig::new().hash_size(8, 8).to_hasher();
    debug!("Hash computed: {:?}", hasher.hash_image(&img));
    Ok(hasher.hash_image(&img))
}

pub fn calculate_similarity(h1: &ImageHash, h2: &ImageHash) -> u32 {
    let dist = h1.dist(h2);
    let similarity = 100 - dist;
    similarity
}
