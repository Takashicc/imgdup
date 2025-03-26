pub mod toast;
use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(message: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-4",
            div { class: "loading loading-spinner loading-xl w-24 h-24" }
            div { class: "text-gray-500 text-sm", "{message}" }
        }
    }
}

#[component]
pub fn ConfirmDialog(
    title: String,
    message: String,
    is_open: Signal<bool>,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<()>,
) -> Element {
    rsx! {
        dialog {
            class: "modal",
            class: if is_open() { "modal-open" },
            div { class: "modal-box",
                h3 { class: "font-bold text-lg", "{title}" }
                div { class: "py-4", "{message}" }
                div { class: "modal-action",
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| {
                            on_cancel.call(());
                            is_open.set(false);
                        },
                        "Cancel"
                    }
                    button {
                        class: "btn btn-warning",
                        onclick: move |_| {
                            on_confirm.call(());
                            is_open.set(false);
                        },
                        "Confirm"
                    }
                }
            }
        }
    }
}
