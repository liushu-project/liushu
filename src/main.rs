use clap::{Parser, Subcommand};
use liushu_core::dict::{compile_dict, query_code};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Compile,

    #[command(arg_required_else_help = true)]
    Query {
        #[arg(long)]
        code: String,

        #[arg(long)]
        page: u32,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Compile => compile_dict().expect("compile error"),
        Commands::Query { code, page } => {
            let result = query_code(code, page).unwrap_or(vec![]);
            println!("{:?}", result);
        }
    };
}
