use dioxus::prelude::*;

pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

pub async fn show_toast(message: &str, toast_type: ToastType) {
    let (alert_class, icon) = match toast_type {
        ToastType::Info => ("alert-info", "ℹ️"),
        ToastType::Success => ("alert-success", "✅"),
        ToastType::Warning => ("alert-warning", "⚠️"),
        ToastType::Error => ("alert-error", "❌"),
    };

    let js = format!(
        r#"
    const alert = document.createElement('div');
    alert.className = 'alert {alert_class}';
    alert.style.transition = 'opacity 0.5s ease-in-out';

    const icon = document.createElement('span');
    icon.textContent = '{icon}';
    icon.style.marginRight = '8px';

    const span = document.createElement('span');
    span.textContent = '{message}';

    alert.appendChild(icon);
    alert.appendChild(span);

    let container = document.querySelector('.toast-container');
    if (!container) {{
        container = document.createElement('div');
        container.className = 'toast-container toast toast-top toast-center';
        container.style.zIndex = '9999';
        document.body.appendChild(container);
    }}

    container.insertAdjacentElement('afterbegin', alert);
    setTimeout(() => {{
        alert.style.opacity = '0';
        setTimeout(() => {{
            alert.remove();
            if (!container.hasChildNodes()) {{
                container.remove();
            }}
        }}, 500);
    }}, 2500);
    "#,
    );
    let _ = dioxus::document::eval(js.as_str()).await;
}

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
