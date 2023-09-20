use clap::Parser;
use cli::{DiffApp, DiffCommand};

mod cli;
mod diff;

fn main() {
    let app = DiffApp::parse();

    match app.command {
        DiffCommand::Diff { first, second } => {
            DiffCommand::handle_diff(&DiffCommand::Diff { first, second });
        }
    }
}
