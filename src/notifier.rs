//! Envia notificações desktop via libnotify (notify-rust).
//! Fallback para stdout se o servidor gráfico não estiver disponível.

use notify_rust::Notification;
use std::env;

pub fn notify(title: &str, message: &str, urgency: &str) {
    let has_display = env::var("WAYLAND_DISPLAY").is_ok() || env::var("DISPLAY").is_ok();

    if has_display {
        let notif = Notification::new()
            .summary(title)
            .body(message)
            .urgency(match urgency {
                "critical" => notify_rust::Urgency::Critical,
                "low" => notify_rust::Urgency::Low,
                _ => notify_rust::Urgency::Normal,
            })
            .show();

        if let Err(e) = notif {
            tracing::warn!("Falha ao enviar notificação: {e}");
            println!("[NOTIFY] {title}: {message}");
        }
    } else {
        println!("[NOTIFY] {title}: {message}");
    }
}