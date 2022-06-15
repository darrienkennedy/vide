mod config;
mod editor;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Action {
    Edit,
    Read,
}

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let conf = config::Config::new();

    let args = Args::parse();
    let editor = editor::Editor::new();

    if matches!(args.action, Action::Edit) {
        editor.edit(args.file, conf);
    } else if matches!(args.action, Action::Read) {
        editor.read(args.file, conf);
    }
}
