use aws_sdk_s3::{Client, Config, Region};
use aws_types::credentials::SharedCredentialsProvider;
use image::DynamicImage;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use tokio::time::{sleep, Duration};
use anyhow::{Result, Context};
use tracing::{info, error, warn};

const R2_BUCKET: &str = "come-here";
const R2_ENDPOINT_URL: &str = "https://34b7b02be8bc4ebdb8c4b34f56c54526.r2.cloudflarestorage.com";
const POLL_INTERVAL: Duration = Duration::from_secs(2);

struct ImageProcessor {
    client: Client,
    previous_files: Vec<String>,
}

impl ImageProcessor {
    fn new(client: Client) -> Self {
        Self {
            client,
            previous_files: Vec::new(),
        }
    }

    async fn list_files(&self) -> Result<Vec<String>> {
        let resp = self.client
            .list_objects_v2()
            .bucket(R2_BUCKET)
            .send()
            .await
            .context("Failed to list objects in R2 bucket")?;

        Ok(resp.contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|obj| obj.key().map(|k| k.to_string()))
            .collect())
    }

    async fn process_file_pair(&self, prefix: &str, files: &[String]) -> Result<()> {
        let dest_folder = format!("temp/images/{}", prefix);
        fs::create_dir_all(&dest_folder)
            .context("Failed to create destination folder")?;

        for file in files {
            info!("Processing file: {}", file);

            self.download_file(file).await?;

            let local_path = format!("{}/{}", dest_folder, 
                Path::new(file).file_name()
                    .context("Invalid filename")?
                    .to_string_lossy());
            
            fs::rename(file, &local_path)
                .context("Failed to move downloaded file")?;

            // Convert to JPEG
            let image = image::open(&local_path)
                .context("Failed to open image")?;
            let jpeg_path = format!("{}.jpg", local_path.trim_end_matches(".png"));
            image.save_with_format(&jpeg_path, image::ImageFormat::Jpeg)
                .context("Failed to save JPEG")?;

            fs::remove_file(&local_path)
                .context("Failed to cleanup original file")?;
        }

        self.run_try_on(prefix, &dest_folder)?;
        Ok(())
    }

    async fn download_file(&self, key: &str) -> Result<()> {
        let get_object = self.client
            .get_object()
            .bucket(R2_BUCKET)
            .key(key)
            .send()
            .await
            .context("Failed to get object from R2")?;

        let data = get_object.body.collect().await?;
        fs::write(key, data.into_bytes())
            .context("Failed to write downloaded file")?;
        Ok(())
    }

    fn run_try_on(&self, prefix: &str, dest_folder: &str) -> Result<()> {
        let model_file = format!("{}/{}model.jpg", dest_folder, prefix);
        let garment_file = format!("{}/{}garment.jpg", dest_folder, prefix);

        if !Path::new(&model_file).exists() || !Path::new(&garment_file).exists() {
            warn!("Missing required files for processing");
            return Ok(());
        }

        let try_on_type = &prefix[..2];
        let script_args = match try_on_type {
            "ha" => vec![
                "run_ootd.py",
                "--model_path", &model_file,
                "--cloth_path", &garment_file,
                "--scale", "0.5",
                "--sample", "1",
            ],
            "to" => vec![
                "run_viton.py",
                "--model", &model_file,
                "--garment", &garment_file,
            ],
            _ => {
                warn!("Unknown try-on type: {}", try_on_type);
                return Ok(());
            }
        };

        Command::new("python")
            .args(&script_args)
            .status()
            .context("Failed to run try-on script")?;

        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            let current_files = self.list_files().await?;

            let new_files: Vec<String> = current_files
                .iter()
                .filter(|file| !self.previous_files.contains(*file))
                .cloned()
                .collect();

            if !new_files.is_empty() {
                let file_groups: HashMap<String, Vec<String>> = new_files
                    .into_iter()
                    .filter_map(|file| {
                        file.split("___")
                            .next()
                            .map(|prefix| (prefix.to_string(), file))
                    })
                    .fold(HashMap::new(), |mut acc, (prefix, file)| {
                        acc.entry(prefix).or_default().push(file);
                        acc
                    });

                for (prefix, files) in file_groups {
                    if files.len() == 2 {
                        if let Err(err) = self.process_file_pair(&prefix, &files).await {
                            error!("Error processing files with prefix {}: {:?}", prefix, err);
                        }
                    }
                }
            } else {
                info!("No new files detected");
            }

            self.previous_files = current_files;
            sleep(POLL_INTERVAL).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let credentials = SharedCredentialsProvider::new("your_access_key", "your_secret_key");
    let config = Config::builder()
        .region(Region::new("auto"))
        .endpoint_url(R2_ENDPOINT_URL)
        .credentials_provider(credentials)
        .build();
    
    let client = Client::from_conf(config);
    let mut processor = ImageProcessor::new(client);

    processor.run().await.context("Processor failed")?;
    Ok(())
}
