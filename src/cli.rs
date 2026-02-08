use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[clap(short, long, default_value = "config.yaml")]
    pub config_path: PathBuf,

    #[clap(short, long)]
    pub output_path: Option<PathBuf>,

    #[clap(long)]
    pub write_default_config: bool,
}
