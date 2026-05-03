use clap::Args;

#[derive(Args, Debug)]
pub struct SysInfoArgs {}

pub struct SysInfoData {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub arch: String,
    pub cpu_brand: String,
    pub cpu_count: usize,
    pub ram_gb: u64,
    pub user: String,
}

pub fn collect_sys_info() -> SysInfoData {
    todo!()
}

pub fn format_sys_info(data: &SysInfoData) -> String {
    format!(
        "Hostname:   {}\nOS:         {}\nKernel:     {}\nArch:       {}\nCPU:        {} ({} cores)\nRAM:        {} GB\nUser:       {}",
        data.hostname,
        data.os,
        data.kernel,
        data.arch,
        data.cpu_brand,
        data.cpu_count,
        data.ram_gb,
        data.user,
    )
}

pub fn run(_args: &SysInfoArgs) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> SysInfoData {
        SysInfoData {
            hostname: "test-host".into(),
            os: "macOS 15.3.1".into(),
            kernel: "24.3.0".into(),
            arch: "aarch64".into(),
            cpu_brand: "Apple M3 Pro".into(),
            cpu_count: 12,
            ram_gb: 36,
            user: "brentm5".into(),
        }
    }

    #[test]
    fn format_contains_all_labels() {
        let output = format_sys_info(&test_data());
        assert!(output.contains("Hostname:"), "missing Hostname label");
        assert!(output.contains("OS:"), "missing OS label");
        assert!(output.contains("Kernel:"), "missing Kernel label");
        assert!(output.contains("Arch:"), "missing Arch label");
        assert!(output.contains("CPU:"), "missing CPU label");
        assert!(output.contains("RAM:"), "missing RAM label");
        assert!(output.contains("User:"), "missing User label");
    }

    #[test]
    fn format_contains_correct_values() {
        let output = format_sys_info(&test_data());
        assert!(output.contains("test-host"));
        assert!(output.contains("macOS 15.3.1"));
        assert!(output.contains("24.3.0"));
        assert!(output.contains("aarch64"));
        assert!(output.contains("Apple M3 Pro"));
        assert!(output.contains("12 cores"));
        assert!(output.contains("36 GB"));
        assert!(output.contains("brentm5"));
    }
}
