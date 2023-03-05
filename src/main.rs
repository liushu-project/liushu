use std::io::{stdin, stdout, Write};

use clap::{Parser, Subcommand};
use liushu_core::deploy::deploy;
use liushu_core::dirs::PROJECT_DIRS;
use liushu_core::engine::Engine;
use liushu_core::hmm::{train, Hmm};
use redb::Database;

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

    #[command(arg_required_else_help = true)]
    Train {
        corpus_file: String,
    },

    Query2,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Deploy => {
            deploy();
        }
        Commands::Query { code } => {
            let engine = Engine::default();
            let result = engine.search(&code).unwrap();
            println!("{:?}", result);
        }
        Commands::Train { corpus_file } => {
            let save_to = &PROJECT_DIRS.target_dir.join("hmm_model.redb");
            train(corpus_file, save_to);
        }
        Commands::Query2 => {
            let db = Database::open(PROJECT_DIRS.target_dir.join("hmm_model.redb")).unwrap();
            let hmm = Hmm::new(db);

            loop {
                print!("hmm> ");
                stdout().flush().unwrap();
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        if input == "*" {
                            break;
                        }

                        hmm.trans(input.trim()).iter().take(8).enumerate().for_each(
                            |(i, result)| {
                                println!("result{}: {}", i, result);
                            },
                        );
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
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
