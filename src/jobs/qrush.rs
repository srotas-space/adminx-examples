// src/jobs/qrush.rs

use std::sync::{Arc};
use tokio::sync::Notify;
use std::env;

use qrush::config::QueueConfig;
use qrush::registry::register_job;
use qrush::config::QUEUE_INITIALIZED;
use qrush::job::Job;
use qrush::config::{set_basic_auth, QrushBasicAuthConfig};
use crate::jobs::notify_user::NotifyUser;

pub async fn initiate(basic_auth: Option<QrushBasicAuthConfig>) {
    let queueNotify = Arc::new(Notify::new());
    let basic_auth = basic_auth.or_else(|| {
        std::env::var("QRUSH_BASIC_AUTH").ok().and_then(|auth| {
            let parts: Vec<&str> = auth.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(QrushBasicAuthConfig {
                    username: parts[0].to_string(),
                    password: parts[1].to_string(),
                })
            } else {
                None
            }
        })
    });

    let _ = set_basic_auth(basic_auth);

    let _ = QUEUE_INITIALIZED.set(queueNotify.clone());

    register_job(NotifyUser::name(), NotifyUser::handler);

    tokio::spawn({
        let queueNotify = queueNotify.clone(); // clone for the async task
        async move {
            let redis_url = std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

            let queues = vec![
                QueueConfig::new("default", 5, 1),
                QueueConfig::new("critical", 10, 0),
            ];

            if let Err(err) = QueueConfig::initialize(redis_url, queues).await {
                eprintln!("❌ Failed to initialize qrush: {:?}", err);
            } else {
                println!("✅ qrush started successfully");
                queueNotify.notify_waiters(); // Notify here after init is complete
            }
        }
    });

    // Wait here until worker calls `notify_waiters()`
    queueNotify.notified().await;
    println!("🚀 Queue initialization complete. Continuing main logic...");
}
