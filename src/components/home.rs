use crate::{
    backend,
    models::{self},
    utils,
};
use std::{collections::HashSet, path::Path};

use dioxus::{
    logger::tracing::{error, info},
    prelude::*,
};

#[component]
pub fn Home() -> Element {
    let mut selected_directory = use_signal(String::new);
    let mut selected_images = use_signal(HashSet::<String>::new);

    let mut similar_images = use_signal(Vec::<models::SimilarImage>::new);
    let mut is_searching = use_signal(|| false);

    rsx! {
        div { class: "container p-4",
            div { class: "pb-4",
                ReferenceImages {  }
            }
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
                    if selected_directory().is_empty() {
                        let js = r#"
                            const toast = document.createElement('div');
                            toast.className = 'toast toast-top toast-center';
                            toast.style.transition = 'opacity 0.5s ease-in-out';

                            const alert = document.createElement('div');
                            alert.className = 'alert alert-warning';

                            const span = document.createElement('span');
                            span.textContent = 'Please select a directory';

                            alert.appendChild(span);
                            toast.appendChild(alert);

                            window.document.body.insertAdjacentElement('afterbegin', toast);
                            setTimeout(() => {
                                toast.style.opacity = '0';
                                setTimeout(() => {
                                    toast.remove();
                                }, 500);
                            }, 2500);
                        "#;
                        document::eval(js).await.unwrap();
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

        div { class: "container p-4",
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

#[component]
fn ReferenceImages() -> Element {
    let mut selected_files = use_signal(Vec::<String>::new);
    let mut is_registering = use_signal(|| false);

    let mut registered_reference_images_resource =
        use_server_future(backend::get_registered_reference_images)?;

    rsx! {
        button {
            class: "btn btn-accent",
            onclick: move |_| {
                document::eval("register_reference_images_modal.showModal()");
                selected_files.write().clear();
            },
            "Check reference images"
        }
        dialog { id: "register_reference_images_modal", class: "modal",
            div { class: "modal-box max-w-2xl",
                div { class: "container p-4",
                    h3 { class: "mb-4", "Register reference images" }
                    div { class: "flex flex-row items-center pb-4",
                        input {
                            r#type: "file",
                            id: "reference-images-input",
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
                            r#for: "reference-images-input",
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
                    button {
                        class: "btn btn-primary w-full mb-4",
                        disabled: is_registering(),
                        onclick: move |_| async move {
                            is_registering.set(true);

                            if let Err(e) = backend::register_reference_images(selected_files()).await {
                                error!("Failed to register reference images: {}", e);
                            }

                            selected_files.write().clear();
                            is_registering.set(false);
                            registered_reference_images_resource.restart();
                        },
                        "Register selected files"
                    }
                }
                div { class: "container p-4",
                    h3 { class: "mb-4", "Registered reference images" }
                    div {
                        {
                            match registered_reference_images_resource.unwrap() {
                                Ok(reference_images) => {
                                    rsx! {
                                        if reference_images.len() > 0 {
                                            for reference_image in reference_images {
                                                RegisteredReferenceImageCard { reference_image, registered_reference_images_resource }
                                            }
                                        } else {
                                            p { class: "text-xs text-error", "No registered reference images. Please register at least one image." }
                                        }
                                    }
                                },
                                Err(e) => {
                                    rsx! {
                                        div { role: "alert", class: "alert alert-error",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                "viewBox": "0 0 24 24",
                                                class: "h-6 w-6 shrink-0 stroke-current",
                                                path {
                                                    "stroke-width": "2",
                                                    "stroke-linejoin": "round",
                                                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                                                    "stroke-linecap": "round",
                                                }
                                            }
                                            span { "Error! Task failed successfully." }
                                        }
                                    }
                                }
                            }
                        }

                    }
                }
            }
            form { method: "dialog", class: "modal-backdrop", button { "close" } }
        }
    }
}

#[component]
fn RegisteredReferenceImageCard(
    reference_image: entity::reference_image::Model,
    registered_reference_images_resource: Resource<
        Result<Vec<entity::reference_image::Model>, ServerFnError>,
    >,
) -> Element {
    let entity::reference_image::Model {
        id,
        filepath,
        created_at,
        updated_at,
        ..
    } = reference_image;

    let path = Path::new(&filepath);
    let parent_path = path
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_default();
    let filename = path
        .file_name()
        .map(|f| f.to_string_lossy())
        .unwrap_or_default();

    let created_at = created_at.format("%Y-%m-%d %H:%M");
    let updated_at = updated_at.format("%Y-%m-%d %H:%M");

    rsx! {
        div { class: "card card-side card-border border-2 mb-4",
            id: "{id}",
            figure { class: "w-24 shrink-0",
                img {
                    class: "rounded-lg object-cover",
                    src: "{utils::path::normalize_path(&filepath)}"
                }
            }
            div { class: "card-body py-3",
                div { class: "space-y-1",
                    p { class: "text-sm text-gray-600 break-all", "{parent_path}"}
                    p { class: "text-base font-semibold break-all", "{filename}"}
                }
                div { class: "flex flex-row justify-between items-center mt-2",
                    div { class: "text-xs text-gray-500",
                        p { "Created: {created_at}" }
                        p { "Updated: {updated_at}" }
                    }
                    button { class: "btn btn-error",
                        onclick: move |_| async move {
                            match backend::delete_registered_reference_image(id).await{
                                Ok(_) => {
                                    registered_reference_images_resource.restart();
                                },
                                Err(e) => {
                                    error!("Failed to delete registered reference image: {}", e);
                                }
                            }
                        },
                        "Unregister"
                    }
                }
            }
        }
    }
}
