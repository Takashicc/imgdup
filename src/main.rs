use image::{imageops::thumbnail, DynamicImage};

use rayon::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};
use walkdir::{DirEntry, WalkDir};

mod utils;

fn main() {
    // TODO Get target image path from cli parameter
    let delete_target_images_hash = get_images_hash("./delete_target_images");
    let target_images_hash = get_images_hash(r"");

    for delete_image_hash in delete_target_images_hash.iter() {
        for target_image_hash in target_images_hash.iter() {
            let similarity = hamming_distance(&delete_image_hash.hash, &target_image_hash.hash);
            if similarity >= 0.99 {
                println!(
                    "Similarity: {} and {}: {}",
                    delete_image_hash.filename, target_image_hash.filename, similarity
                );
            }
        }
    }
}

struct ImageInfo {
    filename: String,
    hash: String,
}

fn get_images_hash(dir_path: &str) -> Vec<ImageInfo> {
    let image_paths = get_image_paths_from_dir(dir_path).unwrap();
    let bar = utils::get_progress_bar(image_paths.len() as u64);
    let msg = format!("Hashing files for: {}", dir_path);
    bar.set_message(msg);

    let images_hash = image_paths
        .par_iter()
        .map(|image_path| {
            let delete_image = image::open(image_path).unwrap();
            let hash = calculate_hash(&delete_image);
            bar.inc(1);
            ImageInfo {
                filename: image_path.clone(),
                hash,
            }
        })
        .collect::<Vec<ImageInfo>>();

    images_hash
}

fn get_image_paths_from_dir(dir_path: &str) -> Result<Vec<String>, std::io::Error> {
    let folder = Path::new(dir_path);
    let mut image_paths = Vec::new();

    if folder.is_dir() {
        let directories = WalkDir::new(folder)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| utils::is_dir(e) && !utils::is_hidden(e))
            .collect::<Vec<DirEntry>>();
        for directory in directories {
            let files = WalkDir::new(directory.path())
                .max_depth(1)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|v| {
                    utils::is_file(v)
                        && !utils::is_hidden(v)
                        && v.path()
                            .extension()
                            .unwrap_or_else(|| OsStr::new(""))
                            .to_string_lossy()
                            .to_lowercase()
                            == "jpg"
                })
                .map(|v| v.into_path())
                .collect::<Vec<PathBuf>>();
            for file in files.iter() {
                if let Some(filename) = file.to_str() {
                    image_paths.push(filename.to_owned());
                }
            }
        }
    }

    Ok(image_paths)
}

fn calculate_hash(image: &DynamicImage) -> String {
    let gray_image = image.to_luma8();
    let resized_image = thumbnail(&gray_image, 8, 8);
    let avg_pixel_value = resized_image.pixels().map(|p| p[0] as u64).sum::<u64>() / 64;
    let hash = resized_image
        .pixels()
        .map(|p| {
            if p[0] as u64 >= avg_pixel_value {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    hash
}

fn hamming_distance(hash1: &str, hash2: &str) -> f32 {
    let len1 = hash1.len();
    let len2 = hash2.len();
    if len1 != len2 {
        panic!("Hash length are not equal")
    }

    let distance = hash1
        .chars()
        .zip(hash2.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count() as f32;

    1.0 - (distance / len1 as f32)
}
