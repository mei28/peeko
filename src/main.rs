use peeko::app::App; // 修正
use peeko::tasks::parser; // 修正
use peeko::ui::run_tui; // 修正

use anyhow::Result;
use clap::Parser;

/// CLI引数を管理
#[derive(Parser)]
struct CliArgs {
    file_path: String,
    #[arg(long)]
    no_tui: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let tasks = parser::parse_tasks(&args.file_path)?;
    let mut app = App::new(tasks, args.file_path.clone())?;

    if args.no_tui {
        for task in app.state.tasks.iter() {
            println!("Found task: {} at line {}", task.name, task.line_number);
        }
        return Ok(());
    }

    let command = run_tui(&mut app)?;

    if let Some(cmd) = command {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut tty = OpenOptions::new().write(true).open("/dev/tty")?;
        write!(tty, "{}", cmd)?;
    }

    Ok(())
}

