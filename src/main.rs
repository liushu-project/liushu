use clap::{Parser, Subcommand};
use liushu_core::deploy::deploy;
use liushu_core::dict::compile_dict;
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
    Compile,

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
        Commands::Compile => compile_dict().expect("compile error"),
        Commands::Query { code } => {
            let engine = SearchEngine::new();
            let result = engine.search(code).unwrap();
            println!("{:?}", result);
        }
    };
}
