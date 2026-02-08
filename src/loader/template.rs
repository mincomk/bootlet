pub fn load_bootlet_config(extra_kernel_opts: &str) -> String {
    let base_config = include_str!("../constant/bootlet.conf");

    base_config.replace("{{ EXTRA_KERNEL_OPTS }}", extra_kernel_opts)
}

pub fn load_loader_config() -> String {
    include_str!("../constant/loader.conf").to_string()
}

pub fn load_init_script(extra_init: &str) -> String {
    include_str!("../constant/init").replace("{{ EXTRA_INIT }}", extra_init)
}

pub fn load_busybox_commands() -> Vec<String> {
    let commands_str = include_str!("../constant/busybox-list.txt");
    commands_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}
