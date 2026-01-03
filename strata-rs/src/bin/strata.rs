use clap::{Parser, Subcommand};
use std::fs;

use strata::decode::decode;
use strata::encode::encode;
use strata::parser::parse;

#[derive(Parser)]
#[command(name = "strata")]
#[command(about = "Strata CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile { input: String, output: String },

    Hash { input: String },

    Decode { input: String },

    Fmt { input: String },
}

fn main() {
    let exit_code = match run() {
        Ok(()) => 0,
        Err(err) => report_error(err),
    };

    std::process::exit(exit_code);
}

fn run() -> Result<(), strata::error::StrataError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output } => {
            let source_text = fs::read_to_string(&input).map_err(strata::error::StrataError::Io)?;

            let ast = parse(&source_text)?;

            let bytecode = encode(&ast)?;

            fs::write(&output, bytecode).map_err(strata::error::StrataError::Io)?;

            Ok(())
        }

        Commands::Hash { input } => {
            let bytecode = if input.ends_with(".st") {
                let source_text =
                    fs::read_to_string(&input).map_err(strata::error::StrataError::Io)?;
                let ast = parse(&source_text)?;
                encode(&ast)?
            } else {
                fs::read(&input).map_err(strata::error::StrataError::Io)?
            };

            let hash = blake3::hash(&bytecode);
            println!("{}", hash.to_hex());

            Ok(())
        }
        Commands::Decode { input } => {
            let bytecode = fs::read(&input).map_err(strata::error::StrataError::Io)?;

            let ast = decode(&bytecode)?;

            println!("{:#?}", ast);

            Ok(())
        }
        Commands::Fmt { input } => {
            let source_text = fs::read_to_string(&input).map_err(strata::error::StrataError::Io)?;

            let ast = parse(&source_text)?;

            println!("{:#?}", ast);

            Ok(())
        }
    }
}

fn report_error(err: strata::error::StrataError) -> i32 {
    use strata::error::StrataError::*;

    match err {
        Parse(e) => {
            eprintln!("error: parse failed");
            eprintln!("reason: {:?}", e.kind);
            eprintln!("line: {}", e.span.line);
            eprintln!("column: {}", e.span.column);
            1
        }

        Decode(e) => {
            eprintln!("error: decode failed");
            eprintln!("reason: {:?}", e.kind);
            eprintln!("offset: {}", e.offset);
            1
        }

        Encode(e) => {
            eprintln!("error: encode failed");
            eprintln!("reason: {:?}", e);
            1
        }

        Io(e) => {
            eprintln!("error: I/O failure");
            eprintln!("reason: {}", e);
            2
        }

        Internal(msg) => {
            eprintln!("error: internal error");
            eprintln!("reason: {}", msg);
            100
        }
    }
}
