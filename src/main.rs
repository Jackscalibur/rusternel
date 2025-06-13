mod modules;

use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

use modules::sysinfo::SystemInfo;
use modules::cpu::CpuInfo;
use modules::processes::ProcessManager;

struct App {
    system_info: SystemInfo,
    cpu_info: CpuInfo,
    process_manager: ProcessManager,
    should_quit: bool,
    last_update: Instant,
    update_interval: Duration,
}

impl Default for App {
    fn default() -> Self {
        let mut system_info = SystemInfo::new();
        system_info.collect_system_info();
        
        let mut cpu_info = CpuInfo::new();
        cpu_info.collect_cpu_info();
        
        let mut process_manager = ProcessManager::new();
        process_manager.refresh_processes();
        
        Self {
            system_info,
            cpu_info,
            process_manager,
            should_quit: false,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(1), // Update every second
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            // Check if it's time to update data
            if self.last_update.elapsed() >= self.update_interval {
                self.refresh_data();
                self.last_update = Instant::now();
            }
            
            terminal.draw(|frame| self.draw(frame))?;
            
            // Use a shorter timeout for event polling to ensure regular updates
            if event::poll(Duration::from_millis(100))? {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn refresh_data(&mut self) {
        self.system_info.collect_system_info();
        self.cpu_info.collect_cpu_info();
        self.process_manager.refresh_processes();
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        
        // Create 2x2 grid layout
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Top half
                Constraint::Percentage(50), // Bottom half
            ])
            .split(area);

        // Split top half horizontally
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Top-left
                Constraint::Percentage(50), // Top-right
            ])
            .split(vertical_chunks[0]);

        // Split bottom half horizontally
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Bottom-left
                Constraint::Percentage(50), // Bottom-right
            ])
            .split(vertical_chunks[1]);

        // Render each section
        self.render_os_info(frame, top_chunks[0]);
        self.render_top_processes(frame, top_chunks[1]);
        self.render_memory_info(frame, bottom_chunks[0]);
        self.render_cpu_info(frame, bottom_chunks[1]);
    }

    fn render_os_info(&self, frame: &mut Frame, area: Rect) {
        // Use existing system_info but create a focused OS/hostname view
        let text = Text::from(vec![
            Line::from(format!("OS: {}", self.system_info.get_os_version())),
            Line::from(format!("Hostname: {}", hostname::get().unwrap_or_default().to_string_lossy())),
            Line::from(format!("Kernel: {}", std::env::consts::ARCH)),
        ]);

        let block = Block::default()
            .title("System")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));

        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, area);
    }

    fn render_top_processes(&self, frame: &mut Frame, area: Rect) {
        // Get actual top processes from the process manager
        let top_processes = self.process_manager.get_top_processes(3);
        
        let mut lines = Vec::new();
        for (i, process) in top_processes.iter().enumerate() {
            lines.push(Line::from(format!(
                "{}. {} (PID: {}) - {:.1}% CPU", 
                i + 1, 
                process.name, 
                process.pid, 
                process.cpu_usage
            )));
        }
        
        // Fill with placeholder if not enough processes
        while lines.len() < 3 {
            lines.push(Line::from(format!("{}. No process data", lines.len() + 1)));
        }

        let text = Text::from(lines);
        let block = Block::default()
            .title("Top Processes")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, area);
    }

    fn render_memory_info(&self, frame: &mut Frame, area: Rect) {
        // Read actual memory info from /proc/meminfo
        let (total, available, used_percent, swap_total, swap_used_percent) = self.get_memory_info();
        
        let text = Text::from(vec![
            Line::from(format!("Total: {:.1} GB", total / 1024.0 / 1024.0)),
            Line::from(format!("Used: {:.1}% ({:.1} GB)", used_percent, (total * used_percent / 100.0) / 1024.0 / 1024.0)),
            Line::from(format!("Available: {:.1} GB", available / 1024.0 / 1024.0)),
            Line::from(format!("Swap: {:.1} GB ({:.1}% used)", swap_total / 1024.0 / 1024.0, swap_used_percent)),
        ]);

        let block = Block::default()
            .title("Memory")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue));

        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, area);
    }

    fn get_memory_info(&self) -> (f32, f32, f32, f32, f32) {
        if let Ok(contents) = std::fs::read_to_string("/proc/meminfo") {
            let mut mem_total = 0.0;
            let mut mem_available = 0.0;
            let mut swap_total = 0.0;
            let mut swap_free = 0.0;

            for line in contents.lines() {
                if line.starts_with("MemTotal:") {
                    mem_total = line.split_whitespace().nth(1)
                        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                } else if line.starts_with("MemAvailable:") {
                    mem_available = line.split_whitespace().nth(1)
                        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                } else if line.starts_with("SwapTotal:") {
                    swap_total = line.split_whitespace().nth(1)
                        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                } else if line.starts_with("SwapFree:") {
                    swap_free = line.split_whitespace().nth(1)
                        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                }
            }

            let used_percent = if mem_total > 0.0 {
                ((mem_total - mem_available) / mem_total) * 100.0
            } else {
                0.0
            };

            let swap_used_percent = if swap_total > 0.0 {
                ((swap_total - swap_free) / swap_total) * 100.0
            } else {
                0.0
            };

            (mem_total, mem_available, used_percent, swap_total, swap_used_percent)
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        }
    }

    fn render_cpu_info(&self, frame: &mut Frame, area: Rect) {
        // Use the actual CPU module instead of placeholder
        self.cpu_info.render(area, frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('r') => {
                // Refresh both system and CPU info
                self.system_info.collect_system_info();
                self.cpu_info.collect_cpu_info();
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
