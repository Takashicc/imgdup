use std::{collections::HashSet, fs, path::Path};

use dioxus::{
    logger::tracing::{debug, info},
    prelude::*,
};

use anyhow::{Context, Result};
use img_hash::{HasherConfig, ImageHash};
use serde::{Deserialize, Serialize};
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG)
        .expect("Failed to initialize logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Content {}
    }
}

// Collect the image paths from the given directory
// It will look for the images in the subdirectories as well
// It will collect the first 5 images and the last 5 images for each directory
// TODO use result
fn scan_images(directory: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for entry in walkdir::WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let mut imgs = fs::read_dir(entry.path())
            .unwrap()
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

    targets
}

// TODO what is 8, 8?
// TODO is it using phash algorithm?
fn compute_hash(path: &str) -> Result<ImageHash> {
    let img = image::open(path).context("Failed to open image")?;
    let hasher = HasherConfig::new().hash_size(8, 8).to_hasher();
    Ok(hasher.hash_image(&img))
}

fn calculate_similarity(h1: &ImageHash, h2: &ImageHash) -> u32 {
    let dist = h1.dist(h2);
    let similarity = 100 - dist;
    similarity
}

fn get_reference_hashes() -> Vec<ImageHash> {
    let mut hashes = Vec::new();
    for entry in walkdir::WalkDir::new("reference_files")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|v| v.file_type().is_file() && v.path().extension().unwrap_or_default().eq("jpg"))
    {
        debug!("Computing hash for: {:?}", entry.path());
        let hash = compute_hash(&entry.path().to_string_lossy().to_string()).unwrap();
        hashes.push(hash);
    }
    hashes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarImage {
    pub filepath: String,
    pub similarity: u32,
}

#[component]
fn Content() -> Element {
    let mut selected_directory = use_signal(String::new);
    let mut selected_images = use_signal(HashSet::<String>::new);

    let mut similar_images = use_signal(Vec::<SimilarImage>::new);

    let mut is_searching = use_signal(|| false);

    rsx! {
        div { class: "container mx-auto p-4",
            div { class: "flex flex-row items-center pb-4",
                input {
                    r#type: "file",
                    id: "custom-input",
                    multiple: false,
                    directory: true,
                    hidden: true,
                    onchange: move |e| {
                        if let Some(file_engine) = e.files() {
                            let files = file_engine.files();
                            if files.len() > 0{
                                selected_directory.set(files[0].to_string());
                            }
                        }
                    },
                }
                label {
                    r#for: "custom-input",
                    class: "btn btn-outline btn-primary mr-3",
                    "Select directory"
                }
                label {
                    class: "text-sm text-slate-500",
                    if selected_directory() == "" {
                        "No directory selected"
                    } else {
                        {selected_directory()}
                    }
                }
            }

            button {
                class: "btn btn-primary w-full",
                disabled: is_searching(),
                onclick: move |_| async move {
                    is_searching.set(true);

                    if let Ok(result) = search_similar_images(selected_directory()).await{
                        similar_images.write().clear();
                        similar_images.write().extend(result);
                    }

                    is_searching.set(false);
                },
                "Search"
            }
        }

        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Duplicated image search results" }
            table { class: "table w-full",
                thead {
                    tr {
                        th { "Select" }
                        th { "Thumbnail" }
                        th { "Filepath" }
                        th { "Similarity" }
                    }
                }
                tbody {
                    {
                        rsx! {
                            if is_searching() {
                                tr {
                                    td { colspan: "4", class: "text-center",
                                        div { class: "loading loading-spinner loading-xl w-32 h-32" }
                                    }
                                }
                            } else {
                                {
                                    similar_images().iter().map(|similar_image| {
                                        let selected_image_filepath = similar_image.filepath.clone();
                                        let filepath = similar_image.filepath.clone();

                                        rsx! {
                                            tr {
                                                td {
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "checkbox",
                                                        onclick: move |_| {
                                                            if selected_images().contains(&selected_image_filepath) {
                                                                selected_images.write().remove(&selected_image_filepath);
                                                            } else {
                                                                selected_images.write().insert(selected_image_filepath.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                                td {
                                                    img {
                                                        src: "{normalize_path(&similar_image.filepath)}",
                                                        class: "w-16 h-16 object-cover",
                                                    }
                                                }
                                                td { class: "cursor-pointer hover:text-blue-500",
                                                    ondoubleclick: move |_| {
                                                        let path = filepath.clone();
                                                        async move {
                                                            let _ = open_folder_in_explorer(path).await;
                                                        }
                                                    },
                                                    "{similar_image.filepath}"
                                                }
                                                td { "{similar_image.similarity}%" }
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: "container mx-auto p-4",
            button {
                class: "btn btn-warning w-full",
                onclick: move |_| {
                    info!("Selected images: {:?}", selected_images());
                },
                "Delete selected images"
            }
        }
        // TODO modal

    }
}

// https://github.com/DioxusLabs/dioxus/issues/1814
// https://github.com/tauri-apps/tauri/blob/f37e97d410c4a219e99f97692da05ca9d8e0ba3a/crates/tauri/scripts/core.js#L17
fn normalize_path(p: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        let p = p.replace("\\", "/");
        format!("http://dioxus.localhost/{p}")
    }
    #[cfg(target_os = "macos")]
    {
        format!("{p}")
    }
}

#[server]
async fn search_similar_images(
    selected_directory: String,
) -> Result<Vec<SimilarImage>, ServerFnError> {
    if selected_directory == "" {
        return Ok(Vec::<SimilarImage>::new());
    }

    let target_files = scan_images(&selected_directory);

    use rayon::prelude::*;
    let reference_hashes = get_reference_hashes();
    let mut calc_results = target_files
        .par_iter()
        .filter_map(|filepath| {
            let hash = compute_hash(filepath).unwrap();
            let max_similarity = reference_hashes
                .iter()
                .map(|ref_hash| calculate_similarity(ref_hash, &hash))
                .fold(0u32, |a, b| a.max(b));
            Some((filepath, max_similarity))
        })
        .collect::<Vec<_>>();

    let mut similar_images = Vec::<SimilarImage>::new();
    calc_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (file, sim) in calc_results.into_iter().filter(|(_, s)| *s >= 90).take(10) {
        similar_images.push(SimilarImage {
            filepath: format!("{}", file.to_string()),
            similarity: sim,
        });
    }
    Ok(similar_images)
}

#[server]
async fn open_folder_in_explorer(path: String) -> Result<(), ServerFnError> {
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
