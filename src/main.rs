use anyhow::Result;
use clap::Parser;
use peeko::app::App;
use peeko::tasks::parser;
use peeko::ui;
use std::fs::OpenOptions;
use std::io::Write;

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
    let mut app = App::new(tasks, args.file_path.clone())?;

    if args.no_tui {
        for task in app.state().tasks.iter() {
            println!("Found task: {} at line {}", task.name, task.line_number);
        }
        return Ok(());
    }

    let command = ui::run_tui(&mut app)?;

    // TUI終了後、コマンドを/dev/ttyに書き込み、あたかもユーザーがタイプしたかのように見せる
    if let Some(cmd) = command {
        // /dev/ttyに書き込み
        let mut tty = OpenOptions::new().write(true).open("/dev/tty")?;
        // コマンド文字列を書き込み（最後にスペースや改行を付けない：ユーザーがEnterを押して実行可能）
        write!(tty, "{}", cmd)?;
        // ここでEnterを自動挿入しないことで、ユーザーは続けてEnterを押せば実行できる
    }

    Ok(())
}

