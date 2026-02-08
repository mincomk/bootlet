# Bootlet

Bootlet is a minimal Linux that boots before the main OS, providing a quick access to simple tools and a selection of operating systems to boot into.

You are dropped into a shell environment less than a second after the computer logo appears. You can pair [Bootit](https://github.com/mincomk/bootit) with Bootlet to get `it windows`, `it linux` command to boot into your favorite OS directly from Bootlet.

## Usage
To create a Bootlet image, use the following command:
```
bootlet -c <config_path> -o <output_img_path>
```
Where `<config_path>` is the path to your Bootlet configuration file and `<output_img_path>` is the desired output `.img` file path. You can then write this image to a USB drive or other bootable media.

## Configuration
Bootlet uses a configuration file to define its behavior and the operating systems available for booting. The configuration file is in **yaml** format. Bootlet is designed to produce identical images for identical configuration.

### Manual Configuration
You can manually create a configuration file in yaml format. Basic template of the configuration file can be generated using:
```
bootlet --write-default-config
```
This will create `config.yaml` in the current directory.

### Suggested Configuration
- [Bootlet Switch](https://github.com/mincomk/bootlet-switch): A physical toggle switch under your table that lets you choose between two different Bootlet configurations at boot time.

### Configuration Options
- `busybox`, `linux_kernel`, `systemd_boot_binary`: Refers to the paths of binaries that always need to be provided. Each field can either be set to `type: auto_download` to let Bootlet download the latest version automatically, or `type: path` with `path: <your path>` to provide a custom binary. Following is example of `busybox` configuration:
  ```yaml
  busybox:
    type: path
    path: "/path/to/your/busybox"
  ```
- `rootfs_size_mb`: Size of the root filesystem in megabytes.
- `extra_init_script`: Extra piece of shell script that will be executed **after OS initialization** of Bootlet. Since this is injected inside init script, you should only include shell commands here. **No `#!/bin/sh`**. Also, this option contains direct text, not a path to a file. Following is an example that adds a custom message to be displayed on boot:
  ```yaml
  extra_init_script: |
    echo "Welcome to Bootlet!"
  ```
- `extra_bin_files`: List of extra binary files to be included in the Bootlet image. Each entry should specify the `file` (path on the host system) and `target` (path inside the Bootlet environment). Following is an example:
  ```yaml
  extra_bin_files:
    - source_path: "./bootit"
      destination_path: "/bin/bootit"
    - source_path: "./it"
      destination_path: "/bin/it"
  ```
