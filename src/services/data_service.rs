// test/src/services/data_service.rs
use anyhow::Result;

// 🎯 IMPORT QRUSH FUNCTIONS
use qrush::queue::{enqueue, enqueue_in};

// 🎯 IMPORT YOUR JOB
use crate::qrushes::jobs::notify_user::NotifyUser;

pub struct DataService;

impl DataService {
    // Simple example: Create a data record and enqueue jobs
    pub async fn create_record(user_id: &str, data: &str) -> Result<String> {
        let record_id = String::from("abc");
        
        println!("📄 Creating data record: {}", record_id);
        
        // 🎯 ENQUEUE IMMEDIATE JOB
        let _ = enqueue(NotifyUser {
            user_id: user_id.to_string(),
            message: "Your data has been saved successfully!".to_string(),
        }).await;

        let _ = enqueue_in(NotifyUser {
            user_id: user_id.to_string(),
            message: "Don't forget to check your saved data!".to_string(),
        }, 10).await;


        let _ = enqueue_in(NotifyUser {
            user_id: user_id.to_string(),
            message: "Don't forget to check your saved data!".to_string(),
        }, 20).await;


        let _ = enqueue_in(NotifyUser {
            user_id: user_id.to_string(),
            message: "Don't forget to check your saved data!".to_string(),
        }, 30).await;
        
        // 🎯 ENQUEUE DELAYED JOB (after 60 seconds)
        let _ = enqueue_in(NotifyUser {
            user_id: user_id.to_string(),
            message: "Don't forget to check your saved data!".to_string(),
        }, 60).await;
        
        Ok(record_id)
    }
}