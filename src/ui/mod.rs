pub mod render;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

use crate::app::App;
use crate::ui::render::draw_ui;

pub fn run_tui(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    while app.is_running() {
        terminal.draw(|f| {
            draw_ui(f, app.state());
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.quit();
                    }
                    KeyCode::Down => {
                        app.next_task();
                    }
                    KeyCode::Up => {
                        app.prev_task();
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(backend, LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // UI直接テストは難しいので、ヘルパー関数があればここでテスト
    // 現時点では空
    #[test]
    fn ui_dummy_test() {
        assert_eq!(1 + 1, 2);
    }
}

