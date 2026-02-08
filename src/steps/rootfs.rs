use cpio::{
    NewcBuilder,
    newc::{ModeFileType, trailer},
};
use std::io::Write;

pub struct CpioRootfsState {
    cpio_buffer: Vec<u8>,
    inode_counter: u32,
}

impl CpioRootfsState {
    fn new() -> Self {
        Self {
            cpio_buffer: Vec::new(),
            inode_counter: 1,
        }
    }

    pub fn new_file(&mut self, path: &str, data: &[u8]) -> anyhow::Result<()> {
        let builder = NewcBuilder::new(path)
            .mode(0o100755) // regular file with rw-r--r-- permissions
            .uid(0)
            .gid(0)
            .nlink(1)
            .mtime(0);

        let mut writer = builder.write(&mut self.cpio_buffer, data.len() as u32);
        writer.write_all(data)?;
        writer.finish()?;

        self.inode_counter += 1;

        Ok(())
    }

    pub fn new_directory(&mut self, path: &str) -> anyhow::Result<()> {
        let builder = NewcBuilder::new(path)
            .mode(0o40755) // directory with rwxr-xr-x permissions
            .uid(0)
            .gid(0)
            .nlink(2)
            .mtime(0);

        let writer = builder.write(&mut self.cpio_buffer, 0);
        writer.finish()?;

        self.inode_counter += 1;

        Ok(())
    }

    pub fn new_symlink(&mut self, path: &str, target: &str) -> anyhow::Result<()> {
        let builder = NewcBuilder::new(path)
            .mode(0o120777) // symlink with rwxrwxrwx permissions
            .set_mode_file_type(ModeFileType::Symlink)
            .uid(0)
            .gid(0)
            .nlink(1)
            .mtime(0);

        let mut writer = builder.write(&mut self.cpio_buffer, target.len() as u32);
        writer.write_all(target.as_bytes())?;
        writer.finish()?;

        self.inode_counter += 1;

        Ok(())
    }

    pub fn finish(mut self) -> anyhow::Result<Vec<u8>> {
        trailer(&mut self.cpio_buffer)?;

        Ok(self.cpio_buffer)
    }
}

/// Installs BusyBox into a CPIO archive buffer. Assuming `bin` directory already exists in the
/// CPIO archive.
pub fn install_busybox(
    state: &mut CpioRootfsState,
    busybox_binary: &[u8],
    busybox_commands: &[String],
) -> anyhow::Result<()> {
    state.new_file("bin/busybox", busybox_binary)?;

    busybox_commands.iter().for_each(|cmd| {
        let symlink_path = format!("bin/{}", cmd);
        state.new_symlink(&symlink_path, "busybox").unwrap();
    });

    Ok(())
}

pub fn create_base_rootfs() -> anyhow::Result<CpioRootfsState> {
    let mut state = CpioRootfsState::new();

    ["bin", "etc", "proc", "syc", "dev", "tmp"]
        .iter()
        .for_each(|dir| {
            state.new_directory(dir).unwrap();
        });

    Ok(state)
}

pub fn install_init_script(
    state: &mut CpioRootfsState,
    script_content: &str,
) -> anyhow::Result<()> {
    state.new_file("/init", script_content.as_bytes())?;

    Ok(())
}

pub fn install_extra_bin_file(
    state: &mut CpioRootfsState,
    target_path: &str,
    file_content: &[u8],
) -> anyhow::Result<()> {
    state.new_file(target_path, file_content)?;

    Ok(())
}

pub fn finalize_rootfs(state: CpioRootfsState) -> anyhow::Result<Vec<u8>> {
    let rootfs_data = state.finish()?;

    Ok(rootfs_data)
}
