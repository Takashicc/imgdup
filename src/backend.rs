use std::path::Path;

use anyhow::{Context, Result};
use dioxus::prelude::*;

use crate::{config, image_processing, models};

#[server]
pub async fn search_similar_images(
    selected_directory: String,
) -> Result<Vec<models::SimilarImage>, ServerFnError> {
    if selected_directory == "" {
        return Ok(Vec::new());
    }

    let target_files = match scan_images(&selected_directory) {
        Ok(v) => v,
        Err(e) => return Err(ServerFnError::new(e.to_string())),
    };

    use rayon::prelude::*;
    let reference_hashes = config::get_reference_hashes();
    let mut calc_results = target_files
        .par_iter()
        .filter_map(|filepath| {
            let hash = image_processing::compute_hash(filepath).unwrap();
            let max_similarity = reference_hashes
                .iter()
                .map(|ref_hash| image_processing::calculate_similarity(ref_hash, &hash))
                .fold(0u32, |a, b| a.max(b));
            Some((filepath, max_similarity))
        })
        .collect::<Vec<_>>();

    let mut similar_images = Vec::new();
    calc_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (file, sim) in calc_results.into_iter().filter(|(_, s)| *s >= 90).take(10) {
        similar_images.push(models::SimilarImage {
            filepath: format!("{}", file.to_string()),
            similarity: sim,
        });
    }
    Ok(similar_images)
}

// Collect the image paths from the given directory
// It will look for the images in the subdirectories as well
// It will collect the first 5 images and the last 5 images for each directory
fn scan_images(directory: &str) -> Result<Vec<String>> {
    let mut targets = Vec::new();
    for entry in walkdir::WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let mut imgs = std::fs::read_dir(entry.path())
            .with_context(|| format!("Failed to read directory: {}", entry.path().display()))?
            .filter_map(Result::ok)
            .filter(|d| {
                matches!(
                    d.path()
                        .extension()
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                        .to_str()
                        .unwrap_or_default(),
                    "jpg" | "jpeg" | "png"
                )
            })
            .map(|d| d.path().to_string_lossy().to_string())
            .collect::<Vec<_>>();

        imgs.sort();

        let selected = imgs
            .iter()
            .take(5)
            .chain(imgs.iter().rev().take(5))
            .cloned()
            .collect::<Vec<_>>();

        targets.extend(selected);
    }

    Ok(targets)
}

#[server]
pub async fn open_folder_in_explorer(path: String) -> Result<(), ServerFnError> {
    let path = Path::new(&path);
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn()
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    Ok(())
}
