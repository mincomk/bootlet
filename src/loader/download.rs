use indicatif::ProgressBar;
use reqwest::blocking::Client;
use std::io::Read;

use crate::config::FileSourceConfig;

pub fn load_file_source(source: &FileSourceConfig, url: &str) -> anyhow::Result<Vec<u8>> {
    match source {
        FileSourceConfig::AutoDownload => {
            println!("Auto-downloading file from {}", url);

            let client = Client::new();

            let mut response = client.get(url).send()?;
            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to download file from {}: HTTP {}",
                    url,
                    response.status()
                ));
            }

            let total_size = response
                .content_length()
                .ok_or_else(|| anyhow::anyhow!("Failed to get content length from response"))?;

            let pb = ProgressBar::new(total_size);
            pb.set_message(format!("Downloading from {}", url));

            let mut downloaded_data = vec![0u8; total_size as usize];

            let mut offset = 0;

            loop {
                let n = response.read(&mut downloaded_data[offset..])?;
                if n == 0 {
                    break;
                }

                offset += n;
                pb.inc(n as u64);
            }

            pb.finish_with_message(format!("Downloaded from {}", url));
            println!("Download completed, size: {} bytes", downloaded_data.len());

            Ok(downloaded_data)
        }
        FileSourceConfig::Path { path } => {
            let data = std::fs::read(path)?;
            Ok(data)
        }
    }
}

pub fn load_local_file(path: &std::path::PathBuf) -> anyhow::Result<Vec<u8>> {
    let data = std::fs::read(path)?;
    Ok(data)
}
