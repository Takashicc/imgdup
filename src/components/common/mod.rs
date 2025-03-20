pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

pub async fn show_toast(message: &str, toast_type: ToastType) {
    let alert_class = match toast_type {
        ToastType::Info => "alert-info",
        ToastType::Success => "alert-success",
        ToastType::Warning => "alert-warning",
        ToastType::Error => "alert-error",
    };

    let js = format!(
        r#"
    const toast = document.createElement('div');
    toast.className = 'toast toast-top toast-center';
    toast.style.transition = 'opacity 0.5s ease-in-out';

    const alert = document.createElement('div');
    alert.className = 'alert {alert_class}';

    const span = document.createElement('span');
    span.textContent = 'Please select a directory';

    alert.appendChild(span);
    toast.appendChild(alert);

    window.document.body.insertAdjacentElement('afterbegin', toast);
    setTimeout(() => {{
        toast.style.opacity = '0';
        setTimeout(() => {{
            toast.remove();
        }}, 500);
    }}, 2500);
    "#,
    );
    let _ = dioxus::document::eval(js.as_str()).await;
}
