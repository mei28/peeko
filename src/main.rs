use anyhow::Result;
use clap::Parser;
use peeko::app::App;
use peeko::tasks::parser;
use peeko::ui;

/// CLI引数
#[derive(Parser)]
struct CliArgs {
    file_path: String,
    #[arg(long)]
    no_tui: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let tasks = parser::parse_tasks(&args.file_path)?;
    let mut app = App::new(tasks, args.file_path)?;

    // no-tuiオプションがある場合は標準出力でタスク一覧を表示
    if args.no_tui {
        for task in app.state().tasks.iter() {
            println!("Found task: {} at line {}", task.name, task.line_number);
        }
        return Ok(());
    }

    // 通常はTUI起動
    ui::run_tui(&mut app)
}

