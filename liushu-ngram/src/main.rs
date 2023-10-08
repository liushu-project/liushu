use clap::{Parser, Subcommand};
use redb::Database;

use liushu_core::hmm::{pinyin_to_sentence, train_to_db};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Train a bigram language model on a text corpus
    Train {
        #[arg(short, long)]
        corpus_path: String,

        #[arg(short, long)]
        output_path: String,
    },

    /// Lookup the most probable hanzi sequence for the input using a trained model
    Lookup {
        inputs: Vec<String>,

        #[arg(short, long)]
        model_path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Train {
            corpus_path,
            output_path,
        }) => {
            let db = Database::create(output_path).unwrap();
            train_to_db(corpus_path, &db).unwrap();
        }
        Some(Commands::Lookup { inputs, model_path }) => {
            let db = Database::open(model_path).unwrap();
            let result = pinyin_to_sentence(inputs, &db).unwrap();
            println!("result is {}", result);
        }
        None => {}
    }
}
