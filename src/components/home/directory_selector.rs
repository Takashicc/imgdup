use dioxus::prelude::*;

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
                label {
                    class: "text-sm text-slate-500",
                    if selected_directory() == "" {
                        "No directory selected"
                    } else {
                        {selected_directory()}
                    }
                }
            }
    }
}
