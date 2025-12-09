// src/utils/s3_util.rs
use aws_sdk_s3::{Client, config::Region, types::ObjectCannedAcl, primitives::ByteStream};
use aws_config;
use aws_credential_types::Credentials;
use dotenv::dotenv;
use std::env;
use anyhow::Result;

pub async fn upload_image_to_s3(file_name: String, content: Vec<u8>) -> Result<String> {
    dotenv().ok();

    let access_key = env::var("AWS_ACCESS_KEY_ID")?;
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY")?;
    let region = env::var("AWS_REGION")?;
    let bucket = env::var("S3_IMAGE_BUCKET")?;

    let credentials = Credentials::new(access_key, secret_key, None, None, "env");
    let shared_config = aws_config::from_env()
        .region(Region::new(region.clone()))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&shared_config);

    let result = client
        .put_object()
        .bucket(&bucket)
        .key(&file_name)
        .body(ByteStream::from(content)) // ✅ use Vec<u8>
        .content_disposition("inline")
        .send()
        .await;

    match result {
        Ok(_) => {
            let public_url = format!("https://{}.s3.{}.amazonaws.com/{}", bucket, region, file_name);
            Ok(public_url)
        }
        Err(err) => {
            eprintln!("❌ S3 upload failed: {:?}", err);
            Err(anyhow::anyhow!("S3 upload failed: {:?}", err))
        }
    }
}
