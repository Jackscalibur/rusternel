use std::collections::HashMap;
use std::fs;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

#[derive(Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub status: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
}

impl Process {
    pub fn new(pid: u32, name: String, status: String) -> Self {
        Process {
            pid,
            name,
            status,
            cpu_usage: 0.0,
            memory_usage: 0.0,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!(
            "PID: {}, Name: {}, Status: {}, CPU: {:.1}%",
            self.pid, self.name, self.status, self.cpu_usage
        );
        buf.set_string(area.x, area.y, text, Style::default());
    }
}

pub struct ProcessManager {
    processes: Vec<Process>,
    previous_cpu_times: HashMap<u32, (u64, u64)>, // PID -> (utime, stime)
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
            previous_cpu_times: HashMap::new(),
        }
    }

    pub fn refresh_processes(&mut self) {
        self.processes.clear();

        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(pid) = file_name.parse::<u32>() {
                        if let Some(process) = self.read_process_info(pid) {
                            self.processes.push(process);
                        }
                    }
                }
            }
        }

        // Sort by CPU usage descending
        self.processes
            .sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
    }

    fn read_process_info(&mut self, pid: u32) -> Option<Process> {
        // Read process name from /proc/PID/comm
        let name = fs::read_to_string(format!("/proc/{}/comm", pid))
            .ok()?
            .trim()
            .to_string();

        // Read process status from /proc/PID/stat
        let stat_content = fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;
        let stat_parts: Vec<&str> = stat_content.split_whitespace().collect();

        if stat_parts.len() < 14 {
            return None;
        }

        let status = stat_parts[2].to_string(); // Process state
        let utime: u64 = stat_parts[13].parse().ok()?; // User time
        let stime: u64 = stat_parts[14].parse().ok()?; // System time

        // Calculate CPU usage (simplified)
        let total_time = utime + stime;
        let cpu_usage = if let Some((prev_utime, prev_stime)) = self.previous_cpu_times.get(&pid) {
            let prev_total = prev_utime + prev_stime;
            if total_time > prev_total {
                ((total_time - prev_total) as f32 / 100.0).min(100.0) // Simplified calculation
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Store current times for next calculation
        self.previous_cpu_times.insert(pid, (utime, stime));

        Some(Process {
            pid,
            name,
            status,
            cpu_usage,
            memory_usage: 0.0, // TODO: implement memory usage calculation
        })
    }

    pub fn get_top_processes(&self, count: usize) -> &[Process] {
        &self.processes[..self.processes.len().min(count)]
    }

    pub fn get_process_count(&self) -> usize {
        self.processes.len()
    }
}