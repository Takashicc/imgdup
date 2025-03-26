use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastType {
    fn get_class(&self) -> &str {
        match self {
            ToastType::Info => "alert-info",
            ToastType::Success => "alert-success",
            ToastType::Warning => "alert-warning",
            ToastType::Error => "alert-error",
        }
    }

    fn get_icon(&self) -> &str {
        match self {
            ToastType::Info => "ℹ️",
            ToastType::Success => "✅",
            ToastType::Warning => "⚠️",
            ToastType::Error => "❌",
        }
    }
}

struct ToastData {
    message: String,
    toast_type: ToastType,
}

#[derive(Clone)]
struct ToastContext {
    toasts: Signal<Vec<ToastData>>,
}

pub fn use_toast_context() {
    use_context_provider(|| ToastContext {
        toasts: Signal::new(vec![]),
    });
}

#[component]
pub fn ToastPortal() -> Element {
    let mut toast_context = use_context::<ToastContext>();

    rsx! {
        div { class: "toast toast-top toast-center", style: "z-index: 9999",
            for (i, toast) in toast_context.toasts.read().iter().rev().enumerate() {
                ToastItem {
                    id: i,
                    message: toast.message.clone(),
                    toast_type: toast.toast_type.clone(),
                    on_remove: move |_| {
                        toast_context.toasts.write().remove(0); // TODO 間違えてる
                    }
                }
            }
        }
    }
}

#[component]
fn ToastItem(
    id: usize,
    message: String,
    toast_type: ToastType,
    on_remove: EventHandler<()>,
) -> Element {
    let toast_class = toast_type.get_class();
    let toast_icon = toast_type.get_icon();

    let mut opacity = use_signal(|| 1.0);

    use_future(move || async move {
        // TODO トランジションがうまくいっておらず、すぐ消えてる気がする
        async_std::task::sleep(std::time::Duration::from_millis(2500)).await;
        opacity.set(0.0);

        async_std::task::sleep(std::time::Duration::from_millis(500)).await;
        on_remove.call(());
    });

    rsx! {
        div {
            key: id,
            class: "alert {toast_class}",
            style: "transition: opacity 0.5s ease-in-out;",
            span {
                "{toast_icon} {message}"
            }
        }
    }
}

pub async fn show_toast(message: &str, toast_type: ToastType) {
    if let Some(mut toast_context) = try_use_context::<ToastContext>() {
        toast_context.toasts.write().push(ToastData {
            message: message.to_string(),
            toast_type,
        });
    }
}
