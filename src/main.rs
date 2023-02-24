use clap::{Parser, Subcommand};
use liushu_core::deploy::deploy;
use liushu_core::dict::compile_dict_to_db;
use liushu_core::search::SearchEngine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Deploy,

    #[command(arg_required_else_help = true)]
    Compile {
        dict: String,
        target: String,
    },

    #[command(arg_required_else_help = true)]
    Query {
        code: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Deploy => {
            deploy();
        }
        Commands::Compile { dict, target } => compile_dict_to_db(dict, target).unwrap(),
        Commands::Query { code } => {
            let engine = SearchEngine::new();
            let result = engine.search(code).unwrap();
            println!("{:?}", result);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
}
