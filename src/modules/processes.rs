use std::fs;
use std::io;

fn list_running_processes() -> Result<Vec<String>, io::Error> {
    let mut processes = Vec::new();
    let entries = fs::read_dir("/proc")?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Ok(pid) = entry.file_name().into_string() {
                if pid.chars().all(|c| c.is_digit(10)) {
                    processes.push(pid);
                }
            }
        }
    }

    Ok(processes)
}

fn filter_processes(processes: Vec<String>, filter: &str) -> Vec<String> {
    processes.into_iter()
        .filter(|pid| pid.contains(filter))
        .collect()
}

// Maybe create a function to send signals to processes?
// A tree view of processes would be cool too, but requires more work
pub fn get_filtered_processes(filter: &str) -> Result<Vec<String>, io::Error> {
    let processes = list_running_processes()?;
    Ok(filter_processes(processes, filter))
}