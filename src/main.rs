use anyhow::Result;
use clap::Parser;

use peeko::app::App;
use peeko::tasks::parser;
use peeko::ui;

#[derive(Parser)]
struct CliArgs {
    file_path: String,
    #[arg(long)]
    no_tui: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse(); // `CliArgs::parse()`が呼べるようになる
    let tasks = parser::parse_tasks(&args.file_path)?;
    let mut app = App::new(tasks, args.file_path)?;

    if args.no_tui {
        for task in app.state().tasks.iter() {
            println!("Found task: {} at line {}", task.name, task.line_number);
        }
        return Ok(());
    }

    ui::run_tui(&mut app)
}

