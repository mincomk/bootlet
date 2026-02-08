mod partition;
mod rootfs;

#[derive(Debug, Clone)]
pub struct RootfsStepConfig<'a> {
    pub busybox_binary: &'a [u8],
    pub busybox_commands: &'a [String],
    pub init_script: String,
    pub extra_bin_files: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Clone)]
pub struct PartitionStepConfig<'a> {
    pub systemd_boot_binary: &'a [u8],
    pub bz_image_binary: &'a [u8],
    pub rootfs_image: &'a [u8],
    pub loader_conf: String,
    pub bootlet_conf: String,
    pub partition_size_mb: u16,
}

pub fn create_rootfs(config: RootfsStepConfig) -> anyhow::Result<Vec<u8>> {
    let mut state = rootfs::create_base_rootfs()?;

    rootfs::install_busybox(&mut state, config.busybox_binary, config.busybox_commands)?;
    rootfs::install_init_script(&mut state, &config.init_script)?;

    for (target_path, file_content) in config.extra_bin_files {
        rootfs::install_extra_bin_file(&mut state, &target_path, &file_content)?;
    }

    let rootfs_image = rootfs::finalize_rootfs(state)?;

    Ok(rootfs_image)
}

pub fn setup_partition(config: PartitionStepConfig) -> anyhow::Result<Vec<u8>> {
    let mut partition_image = partition::create_partition_image(config.partition_size_mb)?;

    {
        let fs = partition::create_fat32_filesystem(&mut partition_image)?;
        let root_dir = fs.root_dir();

        let systemd_boot_config = partition::SystemdBootConfig {
            systemd_boot_binary: config.systemd_boot_binary,
            loader_conf: &config.loader_conf,
            bootlet_conf: &config.bootlet_conf,
        };
        partition::install_bootloader(&root_dir, &systemd_boot_config)?;

        partition::copy_file(&root_dir, config.bz_image_binary, "bzImage")?;

        partition::copy_file(&root_dir, config.rootfs_image, "initrd.img")?;
    }

    Ok(partition_image)
}
