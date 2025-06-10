use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::style::Style;

struct Process {
    pid: u32,
    name: String,
    status: String,
}

impl Process {
    pub fn new(pid: u32, name: String, status: String) -> Self {
        Process { pid, name, status }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        /* This function will render process information in the terminal */
        let text = format!("PID: {}, Name: {}, Status: {}", self.pid, self.name, self.status);
        let line = Line::from(text);
        buf.set_string(area.x, area.y, line, Style::default());
    } 
}