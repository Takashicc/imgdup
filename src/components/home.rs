use crate::{backend, models, utils};
use std::collections::HashSet;

use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Home() -> Element {
    let mut selected_directory = use_signal(String::new);
    let mut selected_images = use_signal(HashSet::<String>::new);

    let mut similar_images = use_signal(Vec::<models::SimilarImage>::new);

    let mut is_searching = use_signal(|| false);

    rsx! {
        ReferenceImages {  }

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

                    if let Ok(result) = backend::search_similar_images(selected_directory()).await{
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
        div { class: "container mx-auto p-4",
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

#[component]
fn ReferenceImages() -> Element {
    let mut selected_files = use_signal(Vec::<String>::new);

    rsx! {
        button {
            class: "btn btn-outline btn-primary",
            onclick: move |_| {
                document::eval("register_reference_images_modal.showModal()");
                selected_files.write().clear();
            },
            "Reference images"
        }
        dialog { id: "register_reference_images_modal", class: "modal",
            div { class: "modal-box",
                h3 { class: "mb-4", "Register reference images" }
                div { class: "flex flex-row items-center pb-4",
                    input {
                        r#type: "file",
                        id: "custom-input",
                        multiple: true,
                        hidden: true,
                        onchange: move |e| {
                            if let Some(file_engine) = e.files() {
                                let files = file_engine.files();
                                selected_files.write().extend(files.iter().map(|file| file.to_string()));
                            }
                        },
                    }
                    label {
                        r#for: "custom-input",
                        class: "btn btn-outline btn-primary mr-3",
                        "Select files"
                    }
                    label {
                        class: "text-sm text-slate-500",
                        if selected_files().len() == 0 {
                            "No files selected"
                        } else {
                            {selected_files().join(", ")}
                        }
                    }
                }
                h3 { "Registered reference images" }
            }
        }
    }
}
