use crate::{
    backend,
    components::{common, reference_images},
    models::{self},
};
use std::collections::HashSet;

use dioxus::{logger::tracing::info, prelude::*};
use search_result::SearchResult;

mod directory_selector;
mod search_result;

#[component]
pub fn Home() -> Element {
    let selected_directory = use_signal(String::new);
    let mut selected_images = use_signal(HashSet::<String>::new);

    let mut similar_images = use_signal(Vec::<models::SimilarImage>::new);
    let mut is_searching = use_signal(|| false);

    let mut is_confirm_dialog_open = use_signal(|| false);

    rsx! {
        div { class: "container p-4",
            div { class: "pb-4",
                reference_images::ReferenceImages {  }
            }
            directory_selector::DirectorySelector { selected_directory }

            button {
                class: "btn btn-primary w-full",
                disabled: is_searching(),
                onclick: move |_| async move {
                    if selected_directory().is_empty() {
                        common::show_toast("Please select a directory", common::ToastType::Info).await;
                        return;
                    }

                    is_searching.set(true);

                    if let Ok(result) = backend::search_similar_images(selected_directory()).await{
                        similar_images.write().clear();
                        similar_images.write().extend(result);
                    }

                    is_searching.set(false);
                },
                "Search"
            }
        }


        SearchResult { is_searching, similar_images, selected_images }

        div { class: "container p-4",
            button {
                class: "btn btn-warning w-full",
                onclick: move |_| async move {
                    if selected_images().is_empty() {
                        common::show_toast("Please select images to delete", common::ToastType::Info).await;
                        return;
                    }

                    is_confirm_dialog_open.set(true);
                },
                "Delete selected images"
            }
        }

        common::ConfirmDialog {
            title: "Are you sure you want to delete the selected images?".to_string(),
            message: "Selected {selected_images().len()} images will be deleted.",
            is_open: is_confirm_dialog_open,
            on_confirm: move |_| async move {
                let selected_similar_images = selected_images().iter().map(|image| image.clone()).collect();
                if let Ok(_) = backend::delete_similar_images(selected_similar_images).await {
                    common::show_toast("Selected images deleted", common::ToastType::Success).await;
                } else {
                    common::show_toast("Failed to delete selected images", common::ToastType::Error).await;
                }

                // TODO make the deleted files unselectable
                selected_images.write().clear();
                is_confirm_dialog_open.set(false);
            },
            on_cancel: move |_| async move {},
        }
    }
}
