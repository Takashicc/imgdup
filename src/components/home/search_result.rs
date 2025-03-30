use std::collections::BTreeMap;
use std::collections::HashSet;

use dioxus::prelude::*;

use crate::backend;
use crate::components::common;
use crate::models;
use crate::utils;

#[component]
pub fn SearchResult(
    is_searching: ReadOnlySignal<bool>,
    similar_images: ReadOnlySignal<BTreeMap<u32, models::SimilarImage>>,
    selected_images: Signal<HashSet<u32>>,
) -> Element {
    rsx! {
        div { class: "container p-4",
            h1 { class: "text-2xl font-bold mb-4", "Duplicated image search results" }

            if is_searching() {
                common::LoadingSpinner{ message: "Searching for duplicated images..." }
            } else if similar_images().is_empty() {
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
                            rsx! {
                                for (image_id, similar_image) in similar_images() {
                                    SearchResultRow { image_id, similar_image, selected_images }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SearchResultRow(
    image_id: u32,
    similar_image: models::SimilarImage,
    selected_images: Signal<HashSet<u32>>,
) -> Element {
    rsx! {
        tr {
            class: if similar_image.is_deleted && similar_image.error_message.is_none() { "opacity-50" },
            class: if similar_image.error_message.is_some() { "bg-red-100" },
            td {
                input {
                    r#type: "checkbox",
                    class: "checkbox",
                    disabled: similar_image.is_deleted,
                    checked: selected_images().contains(&image_id),
                    onclick: move |_| {
                        if selected_images().contains(&image_id) {
                            selected_images.write().remove(&image_id);
                        } else {
                            selected_images.write().insert(image_id.clone());
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
                    let path = similar_image.filepath.clone();
                    async move {
                        if let Err(e) = backend::open_folder_in_explorer(path).await {
                            common::show_toast(e.to_string().as_str(), common::ToastType::Error).await;
                        }
                    }
                },
                "{similar_image.filepath}"
            }
            td {
                "{similar_image.similarity}%"
                if let Some(error_message) = &similar_image.error_message {
                    div { class: "text-red-500 text-sm", "{error_message}"}
                }
            }
        }
    }
}
