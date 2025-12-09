// src/qrushes/crons/daily_report_job.rs
use async_trait::async_trait;
use futures::future::BoxFuture;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use qrush::job::Job;
use qrush::cron::cron_job::CronJob;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReportJob {
    pub report_type: String,
}

#[async_trait]
impl Job for DailyReportJob {
    async fn perform(&self) -> Result<()> {
        println!("Generating {} report...", self.report_type);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let text = format!("{} report generated successfully", self.report_type);
        send_slack_notification(&text).await?;
        println!("{} report generated successfully", self.report_type);
        Ok(())
    }

    fn name(&self) -> &'static str { "DailyReportJob" }

    fn queue(&self) -> &'static str { "default" }
}

#[async_trait]
impl CronJob for DailyReportJob {
    fn cron_expression(&self) -> &'static str {
        "0 * * * * *"
    }

    fn cron_id(&self) -> &'static str { "daily_report" }
}

impl DailyReportJob {
    pub fn name() -> &'static str { "DailyReportJob" }

    pub fn handler(payload: String) -> BoxFuture<'static, Result<Box<dyn Job>>> {
        Box::pin(async move {
            let job: DailyReportJob = serde_json::from_str(&payload)?;
            Ok(Box::new(job) as Box<dyn Job>)
        })
    }
}




async fn send_slack_notification(text: &str) -> Result<()> {
    use anyhow::Context;

    let webhook_url = std::env::var("SLACK_WEBHOOK_URL")
        .context("SLACK_WEBHOOK_URL not set")?;

    let client = reqwest::Client::new();
    let payload = serde_json::json!({ "text": text });

    let resp = client
        .post(&webhook_url)
        .json(&payload) // ✅ works because `json` feature is enabled
        .send()
        .await
        .context("Failed to send request to Slack webhook")?;

    Ok(())
}
