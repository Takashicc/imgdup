use std::collections::HashSet;

use dioxus::prelude::*;

use crate::backend;
use crate::models;
use crate::utils;

#[component]
pub fn SearchResult(
    is_searching: ReadOnlySignal<bool>,
    similar_images: ReadOnlySignal<Vec<models::SimilarImage>>,
    selected_images: Signal<HashSet<String>>,
) -> Element {
    rsx! {
        div { class: "container p-4",
            h1 { class: "text-2xl font-bold mb-4", "Duplicated image search results" }

            if is_searching() {
                div { class: "flex flex-col items-center gap-4",
                    div { class: "loading loading-spinner loading-xl w-24 h-24" }
                    div { class: "text-gray-500 text-sm", "Searching for duplicated images..." }
                }
            } else if similar_images.is_empty() {
                div { class: "text-left text-gray-500", "No results" }
            } else {
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
                                                src: "{utils::path::normalize_path(&similar_image.filepath)}",
                                                class: "w-16 h-16 object-cover",
                                            }
                                        }
                                        td { class: "cursor-pointer hover:text-blue-500",
                                            ondoubleclick: move |_| {
                                                let path = filepath.clone();
                                                async move {
                                                    let _ = backend::open_folder_in_explorer(path).await;
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
