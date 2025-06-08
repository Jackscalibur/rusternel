mod modules;
use std::io::{self, stdout};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // TUI rendering loop
    terminal.draw(|f| {
        // Here you would call your rendering functions, e.g.:
        // modules::sysinfo::render(f);
        f.render_widget(ratatui::widgets::Block::default().title("System Information").borders(ratatui::widgets::Borders::ALL), f.size());
    })?;

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}