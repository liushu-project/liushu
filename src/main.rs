use std::io;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::{params, Connection};
use serde::Deserialize;

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
        code: String,
    },
}

#[derive(Debug, Deserialize)]
struct DictItem {
    text: String,
    code: String,
    weight: u64,
    stem: Option<String>,
    comment: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Compile => compile_dict().expect("compile error"),
        Commands::Query { code } => {
            let result = query_code(code).unwrap_or(vec![]);
            println!("{:?}", result);
        }
    };
}

fn query_code(code: String) -> Result<Vec<String>> {
    let conn = Connection::open("./sunman.db3")?;
    let mut stmt =
        conn.prepare("SELECT text FROM sunman WHERE code LIKE ? ORDER BY weight DESC")?;

    let mut query_code = code;
    query_code.push('%');
    let rows = stmt.query_map(params![query_code], |row| row.get("text"))?;

    let mut result = Vec::new();
    for text_result in rows {
        result.push(text_result?);
    }

    Ok(result)
}

fn compile_dict() -> Result<()> {
    let mut conn = Connection::open("./sunman.db3")?;
    conn.execute(
        "CREATE TABLE sunman (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            code TEXT NOT NULL,
            weight INTEGER NOT NULL,
            stem TEXT,
            comment TEXT
        )",
        (),
    )?;
    let tx = conn.transaction()?;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let dict: DictItem = result?;
        tx.execute(
            "INSERT INTO sunman (text, code, weight, stem, comment) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![dict.text, dict.code, dict.weight, dict.stem, dict.comment],
        )?;
    }
    tx.commit()?;
    Ok(())
}
