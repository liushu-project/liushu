use clap::{Parser, Subcommand};
use liushu_core::deploy::deploy;
use liushu_core::engine::Engine;

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
        Commands::Query { code } => {
            let engine = Engine::new();
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
