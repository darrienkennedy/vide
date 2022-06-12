mod editor;

use clap::Parser;

#[derive(clap::Subcommand)]
enum Action {
    Edit,
    Read,
}

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();
    let editor = editor::Editor::new();

    let target_file = "/tmp/hello".to_string();
    if matches!(args.action, Action::Edit) {
        editor.edit(target_file);
    } else if matches!(args.action, Action::Read) {
        editor.read(target_file);
    }
}
