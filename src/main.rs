use clap::Parser;

use crate::{cli::CliArgs, constant::BUSYBOX_DOWNLOAD_URL};

mod config;
mod constant;
mod loader;

mod steps;

mod cli;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    if args.write_default_config {
        config::save_config(&Default::default(), &args.config_path)?;
        println!(
            "Default configuration written to {}",
            args.config_path.display()
        );
        return Ok(());
    }

    let output_path = match args.output_path {
        Some(path) => path,
        None => {
            return Err(anyhow::anyhow!(
                "Output path must be specified with --output when not writing default config"
            ));
        }
    };

    let config = config::load_config(&args.config_path)?;

    let rootfs_step_config = steps::RootfsStepConfig {
        busybox_binary: &loader::load_file_source(&config.busybox, BUSYBOX_DOWNLOAD_URL)?,
        busybox_commands: &loader::load_busybox_commands(),
        init_script: loader::load_init_script(&config.extra_init_script.unwrap_or_default()),
        extra_bin_files: config
            .extra_bin_files
            .iter()
            .map(|item| {
                loader::load_local_file(&item.file).map(|content| (item.target.clone(), content))
            })
            .collect::<anyhow::Result<Vec<(String, Vec<u8>)>>>()?,
    };

    let partition_step_config = steps::PartitionStepConfig {
        systemd_boot_binary: &loader::load_file_source(
            &config.systemd_boot_binary,
            constant::SYSTEMD_BOOT_DOWNLOAD_URL,
        )?,
        bz_image_binary: &loader::load_file_source(
            &config.linux_kernel,
            constant::BZIMAGE_DOWNLOAD_URL,
        )?,
        loader_conf: loader::load_loader_config(),
        bootlet_conf: loader::load_bootlet_config(&config.extra_kernel_cmdline.unwrap_or_default()),
        partition_size_mb: config.rootfs_size_mb,
    };

    let partition_image = steps::run_steps(partition_step_config, rootfs_step_config)?;

    std::fs::write(&output_path, partition_image)?;

    println!("Partition image written to {}", output_path.display());

    Ok(())
}
