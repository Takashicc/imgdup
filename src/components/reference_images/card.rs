use std::path::Path;

use crate::{backend, utils};
use dioxus::{logger::tracing::error, prelude::*};

#[component]
pub fn RegisteredReferenceImageCard(
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
