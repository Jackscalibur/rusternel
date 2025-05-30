mod modules;
use modules::sysinfo::SysInfo;
use std::time::Duration;

// Useful for printing durations in a human-readable format
fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    format!("{}d {}h {}m {}s", days, hours % 24, mins % 60, secs % 60)
}

fn main() {
    let info = SysInfo::gather().unwrap();

    // Dummy data for demonstration and brainstorming
    println!("Kernel Version: {}", info.kernel_version);
    println!("Uptime: {}", format_duration(info.uptime));
    println!("Load Average: {:?}", info.load_average);
}
