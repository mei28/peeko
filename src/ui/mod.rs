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
    // RAWモード有効化
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // メインループ
    while app.is_running() {
        // 描画
        terminal.draw(|f| {
            draw_ui(f, app.state());
        })?;

        // イベント待ち
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => match key.code {
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
                },
                _ => {}
            }
        }
    }

    // 後処理
    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(backend, LeaveAlternateScreen)?;
    Ok(())
}
