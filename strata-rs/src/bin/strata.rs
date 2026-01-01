use clap::{Parser, Subcommand};
use std::fs;

use strata::parser::parse;
use strata::encode::encode_value;

#[derive(Parser)]
#[command(name = "strata")]
#[command(about = "Strata CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        input: String,
        output: String,
    },

    Hash {
        input: String,
    },

    Fmt {
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output } => {
            let source_text = fs::read_to_string(&input)
                .expect("failed to read input");

            let parsed_value = parse(&source_text)
                .expect("parse failed");

            let bytes = encode_value(&parsed_value);

            fs::write(&output, bytes)
                .expect("failed to write output");
        }

        Commands::Hash { input } => {
            let bytes = if input.ends_with(".st") {
                let source_text = fs::read_to_string(&input).expect("failed to read input");
                let value = parse(&source_text).expect("parse failed");
                encode_value(&value)
            } else {
                fs::read(&input).expect("failed to read input")
            };

            let hash = blake3::hash(&bytes);

            println!("{}", hash.to_hex());
        }

        Commands::Fmt { input } => {
            let source_text = fs::read_to_string(&input).expect("failed to read input");
            let value = parse(&source_text).expect("parse failed");
            println!("{:#?}", value);
        }
    }
}
