use crate::image_processing;
use anyhow::Result;
use dioxus::logger::tracing::debug;
use img_hash::ImageHash;

pub fn get_reference_hashes() -> Vec<ImageHash> {
    let mut hashes = Vec::new();
    for entry in walkdir::WalkDir::new("reference_files")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|v| v.file_type().is_file() && v.path().extension().unwrap_or_default().eq("jpg"))
    {
        debug!("Computing hash for: {:?}", entry.path());
        let hash =
            image_processing::compute_hash(&entry.path().to_string_lossy().to_string()).unwrap();
        hashes.push(hash);
    }
    hashes
}
