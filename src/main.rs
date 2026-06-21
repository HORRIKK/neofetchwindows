use sysinfo::System;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use std::process::Command;

fn main() {
    let uptime_minutes = System::uptime() / 60;

    let mut sys = System::new();
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let os = format!("OS: {} {}", System::name().unwrap_or_default(), System::os_version().unwrap_or_default());
    let host = format!("Host: {}", System::host_name().unwrap_or_default());
    let kernel = format!("Kernel: {}", System::kernel_version().unwrap_or_default());
    let uptime = format!("Uptime: {} min", uptime_minutes);

    let res_width: i32;
    let res_height: i32;
    unsafe {
        res_width = GetSystemMetrics(SM_CXSCREEN);
        res_height = GetSystemMetrics(SM_CYSCREEN);
    }
    let resolution = if res_width > 0 && res_height > 0 {
        format!("Resolution: {}x{}", res_width, res_height)
    } else {
        "Resolution: Unknown".to_string()
    };

    let cpu_name = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown");
    let cpu_info = format!("CPU: {}", cpu_name);
    let cpu_arch = format!("CPU Arch: {}", System::cpu_arch());

    let gpu_name = Command::new("wmic")
        .args(&["path", "win32_VideoController", "get", "name"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .map(|text| {
            text.lines()
                .skip(1)
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|_| "Unknown GPU".to_string());
    let gpu_info = format!("GPU: {}", gpu_name);

    const BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;
    let mem_available = sys.available_memory() as f64 / BYTES_TO_GB;
    let mem_total = sys.total_memory() as f64 / BYTES_TO_GB;

    let mem_avail_str = format!("Memory available: {:.2} GB", mem_available);
    let mem_total_str = format!("Memory total: {:.2} GB", mem_total);

    let info_lines = [
        os, host, kernel, uptime, resolution, cpu_info, cpu_arch, gpu_info, mem_avail_str, mem_total_str
    ];

    let logo_lines = [
        "\x1b[36m  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "                                       ",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  █████████████████   █████████████████\x1b[0m",
        "\x1b[36m  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀\x1b[0m",
    ];

    let empty_logo_padding = "                                       ";

    let max_lines = std::cmp::max(logo_lines.len(), info_lines.len());
    for i in 0..max_lines {
        let logo_part = logo_lines.get(i).unwrap_or(&empty_logo_padding);
        let info_part = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        println!("{}   {}", logo_part, info_part);
    }
}
