pub mod render;
pub mod events;

use anyhow::Result;
use crate::app::App;

/// TUI のメインループを実行
pub fn run_tui(app: &mut App) -> Result<Option<String>> {
    events::handle_events(app)
}

