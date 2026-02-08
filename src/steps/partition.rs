use fatfs::{Dir, FileSystem, FsOptions, ReadWriteSeek};
use std::io::{Cursor, Write};

#[derive(Debug, Clone)]
pub struct SystemdBootConfig<'a> {
    pub systemd_boot_binary: &'a [u8],
    pub loader_conf: &'a str,
    pub bootlet_conf: &'a str,
}

pub fn install_bootloader(
    root_dir: &Dir<'_, impl ReadWriteSeek>,
    config: &SystemdBootConfig<'_>,
) -> anyhow::Result<()> {
    let efi_dir = root_dir.create_dir("EFI")?;

    let boot_dir = efi_dir.create_dir("BOOT")?;
    let systemd_dir = efi_dir.create_dir("systemd")?;
    let loader_dir = root_dir.create_dir("loader")?;

    // /EFI/BOOT/BOOTX64.EFI
    let mut bootx64_file = boot_dir.create_file("BOOTX64.EFI")?;
    bootx64_file.truncate()?;
    bootx64_file.write_all(config.systemd_boot_binary)?;

    // /EFI/systemd/systemd-bootx64.efi
    let mut systemd_boot_file = systemd_dir.create_file("systemd-bootx64.efi")?;
    systemd_boot_file.truncate()?;
    systemd_boot_file.write_all(config.systemd_boot_binary)?;

    // /loader/loader.conf
    let mut loader_conf_file = loader_dir.create_file("loader.conf")?;
    loader_conf_file.truncate()?;
    loader_conf_file.write_all(config.loader_conf.as_bytes())?;

    // /loader/entries/bootlet.conf
    let entries_dir = loader_dir.create_dir("entries")?;
    let mut bootlet_conf_file = entries_dir.create_file("bootlet.conf")?;
    bootlet_conf_file.truncate()?;
    bootlet_conf_file.write_all(config.bootlet_conf.as_bytes())?;

    Ok(())
}

pub fn copy_file(
    root_dir: &Dir<'_, impl ReadWriteSeek>,
    binary: &[u8],
    target_filename: &str,
) -> anyhow::Result<()> {
    let mut kernel_file = root_dir.create_file(target_filename)?;
    kernel_file.truncate()?;
    kernel_file.write_all(binary)?;

    Ok(())
}

pub fn create_partition_image(partition_size_mb: u16) -> anyhow::Result<Vec<u8>> {
    let partition_size_bytes = (partition_size_mb as usize) * 1024 * 1024;
    let partition_image = vec![0u8; partition_size_bytes];

    Ok(partition_image)
}

pub fn create_fat32_filesystem(
    partition_image: &mut [u8],
) -> anyhow::Result<FileSystem<impl ReadWriteSeek>> {
    let mut cursor = Cursor::new(partition_image);

    fatfs::format_volume(
        &mut cursor,
        fatfs::FormatVolumeOptions::new().fat_type(fatfs::FatType::Fat32),
    )?;

    cursor.set_position(0);

    let fs = FileSystem::new(cursor, FsOptions::new())?;

    Ok(fs)
}
