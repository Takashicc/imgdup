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
    let selected_images = use_signal(HashSet::<String>::new);

    let mut similar_images = use_signal(Vec::<models::SimilarImage>::new);
    let mut is_searching = use_signal(|| false);

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
                onclick: move |_| {
                    info!("Selected images: {:?}", selected_images());
                },
                "Delete selected images"
            }
        }
    }
}
