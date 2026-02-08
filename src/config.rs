use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileSourceConfig {
    #[default]
    AutoDownload,
    Path {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraFile {
    pub target: String,
    pub file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub busybox: FileSourceConfig,
    pub linux_kernel: FileSourceConfig,
    pub systemd_boot_binary: FileSourceConfig,

    pub rootfs_size_mb: u16,

    pub extra_kernel_cmdline: Option<String>,
    pub extra_init_script: Option<String>,
    pub extra_bin_files: Vec<ExtraFile>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            busybox: FileSourceConfig::AutoDownload,
            linux_kernel: FileSourceConfig::AutoDownload,
            systemd_boot_binary: FileSourceConfig::AutoDownload,
            rootfs_size_mb: 512,
            extra_kernel_cmdline: Some(String::from("quiet")),
            extra_init_script: None,
            extra_bin_files: vec![],
        }
    }
}

pub fn load_config(config_path: &PathBuf) -> anyhow::Result<Config> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    Ok(config)
}

pub fn save_config(config: &Config, config_path: &PathBuf) -> anyhow::Result<()> {
    let config_content = serde_yaml::to_string(config)?;
    std::fs::write(config_path, config_content)?;

    Ok(())
}
