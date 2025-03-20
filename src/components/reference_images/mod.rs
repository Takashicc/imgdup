use crate::backend;
use dioxus::{logger::tracing::error, prelude::*};

mod card;

#[component]
pub fn ReferenceImages() -> Element {
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
                                                card::RegisteredReferenceImageCard { reference_image, registered_reference_images_resource }
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
