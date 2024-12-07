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

/// TUI のイベントループを管理
pub fn handle_events(app: &mut App) -> Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut command_to_run = None;

    while app.is_running() {
        terminal.draw(|f| draw_ui(f, &app.state))?; 

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('j') => app.next_task(),
                    KeyCode::Down => app.next_task(),
                    KeyCode::Char('k') => app.prev_task(),
                    KeyCode::Up => app.prev_task(),
                    KeyCode::Enter => {
                        let state = &app.state;
                        if let Some(cmd) = state
                            .tasks
                            .get(state.selected_index)
                            .and_then(|t| t.command.clone())
                        {
                            command_to_run = Some(cmd);
                        }
                        app.quit();
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(command_to_run)
}
