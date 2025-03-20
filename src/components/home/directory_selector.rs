use dioxus::prelude::*;
use dioxus_free_icons::{icons, Icon};

#[component]
pub fn DirectorySelector(selected_directory: Signal<String>) -> Element {
    rsx! {
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
                div { class: "text-sm text-slate-500",
                    if selected_directory() == "" {
                        div { "No directory selected" }
                    } else {
                        div { class: "inline-block p-1 group rounded hover:bg-slate-100 transition-colors duration-100",
                            span { class: "break-all",
                                "{selected_directory()}",
                                span { class: "whitespace-nowrap",
                                    button {
                                        class: "btn btn-xs btn-circle bg-gray-300 border-none ml-1 opacity-0 group-hover:opacity-100 transition-opacity duration-100 hover:bg-red-500",
                                        onclick: move |_| selected_directory.set("".to_string()),
                                        Icon {
                                            fill: "white",
                                            icon: icons::ld_icons::LdX,
                                        },
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
}
