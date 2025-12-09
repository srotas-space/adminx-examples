// test/src/qrushes/jobs/notify_user.rs

use qrush::job::Job;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Error};
use futures::future::{BoxFuture, FutureExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyUser {
    pub user_id: String,
    pub message: String,
}

#[async_trait]
impl Job for NotifyUser {
    fn name(&self) -> &'static str {
        "NotifyUser"
    }

    fn queue(&self) -> &'static str {
        "default"
    }

    async fn before(&self) -> Result<()> {
        println!("⏳ Before NotifyUser job for user: {}", self.user_id);
        Ok(())
    }

    async fn perform(&self) -> Result<()> {
        // Your code here
        println!("📬 Performing NotifyUser: '{}' to user {}", self.message, self.user_id);
        Ok(())
    }

    async fn after(&self) {
        println!("✅ After NotifyUser job for user: {}", self.user_id);
    }

    async fn on_error(&self, err: &Error) {
        eprintln!("❌ Error in NotifyUser job for user {}: {:?}", self.user_id, err);
    }

    async fn always(&self) {
        println!("🔁 Always block executed for NotifyUser job");
    }
}


impl NotifyUser {
    pub fn name() -> &'static str {
        "notify_user"
    }

    //  handler signature matching registry
    pub fn handler(payload: String) -> BoxFuture<'static, Result<Box<dyn Job>>> {
        async move {
            let job: NotifyUser = serde_json::from_str(&payload)?;
            Ok(Box::new(job) as Box<dyn Job>)
        }
        .boxed()
    }
}
