use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
pub struct DiffApp {
    #[clap(subcommand)]
    pub command: DiffCommand,
}

#[derive(Debug, Subcommand)]
pub enum DiffCommand {
    #[clap(
        about = "Compare two files",
        long_about = "Compare two files and show the differences"
    )]
    Diff {
        #[clap(help = "First file")]
        first: String,
        #[clap(help = "Second file")]
        second: String,
    },
}
impl DiffCommand {
    pub fn handle_diff(&self) {
        match self {
            DiffCommand::Diff { first, second } => {
                println!("Comparing {} and {}", first, second);
            }
        }
    }
}
