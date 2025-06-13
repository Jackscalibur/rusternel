use std::env::consts;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct SystemInfo {
    os_version: String,
    cpu_info: String,
    memory_info: String,

}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            os_version: String::new(),
            cpu_info: String::new(),
            memory_info: String::new(),
        }
    }

    pub fn collect_system_info(&mut self) {
        self.os_version = format!("{} {}", consts::OS, consts::ARCH);
        self.cpu_info = "CPU Info: Placeholder for CPU details".to_string(); // Placeholder for actual CPU info collection
        self.memory_info = "Memory Info: Placeholder for memory details".to_string(); // Placeholder for actual memory info collection
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        /* This function will render the system information in the terminal */
        let block = Block::default()
            .title("System Information")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));
        
        block.render(area, buf);

        let text = Text::from(vec![
            Line::from(format!("OS Version: {}", self.os_version)),
            Line::from(format!("CPU Info: {}", self.cpu_info)),
            Line::from(format!("Memory Info: {}", self.memory_info)),
        ]);

        // Recreate the block for the Paragraph to avoid use-after-move
        let paragraph_block = Block::default()
            .title("System Information")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        Paragraph::new(text)
            .block(paragraph_block)
            .render(area, buf);
    }

    pub fn get_os_version(&self) -> &str {
        &self.os_version
    }
}