use std::fs;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    style::{Color, Style},
};

pub struct CpuInfo {
    cpu_usage: f32,
    load_avg: (f32, f32, f32),
    core_count: usize,
    thread_count: usize,
    uptime: String,
}

impl CpuInfo {
    pub fn new() -> Self {
        CpuInfo {
            cpu_usage: 0.0,
            load_avg: (0.0, 0.0, 0.0),
            core_count: 0,
            thread_count: 0,
            uptime: String::new(),
        }
    }

    pub fn collect_cpu_info(&mut self) {
        // Read CPU info from /proc/cpuinfo
        if let Ok(contents) = fs::read_to_string("/proc/cpuinfo") {
            self.core_count = contents.lines()
                .filter(|line| line.starts_with("processor"))
                .count();
            
            // For simplicity, assume 2 threads per core (hyperthreading)
            self.thread_count = self.core_count * 2;
        }

        // Read load averages from /proc/loadavg
        if let Ok(contents) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = contents.split_whitespace().collect();
            if parts.len() >= 3 {
                self.load_avg = (
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                );
            }
        }

        // Read uptime from /proc/uptime
        if let Ok(contents) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime_seconds) = contents.split_whitespace().next() {
                if let Ok(seconds) = uptime_seconds.parse::<f64>() {
                    let days = (seconds / 86400.0) as u64;
                    let hours = ((seconds % 86400.0) / 3600.0) as u64;
                    let minutes = ((seconds % 3600.0) / 60.0) as u64;
                    self.uptime = format!("{}d {}h {}m", days, hours, minutes);
                }
            }
        }

        // Placeholder for CPU usage calculation (would need multiple samples)
        self.cpu_usage = 23.5; // This would require calculating from /proc/stat
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("CPU & Load")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(Style::default().fg(Color::Red));

        let text = Text::from(vec![
            Line::from(format!("CPU Usage: {:.1}%", self.cpu_usage)),
            Line::from(format!("Load Avg: {:.1}, {:.1}, {:.1}", 
                self.load_avg.0, self.load_avg.1, self.load_avg.2)),
            Line::from(format!("Cores: {} ({} threads)", self.core_count, self.thread_count)),
            Line::from(format!("Uptime: {}", self.uptime)),
        ]);

        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }

    // Getter methods for use in main.rs
    pub fn get_cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn get_load_avg(&self) -> (f32, f32, f32) {
        self.load_avg
    }

    pub fn get_core_count(&self) -> usize {
        self.core_count
    }

    pub fn get_uptime(&self) -> &str {
        &self.uptime
    }
}