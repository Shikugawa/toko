use std::{env, process::exit};

use clap::{Parser, Subcommand};
use snippet::{edit_snippets, load_snippets, SnippetsSearcher};

pub mod snippet;

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[clap(name = "toko")]
#[clap(about = "A snippet manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Edit,
    Search,
}

fn main() {
    let args = Cli::parse();

    let home = match env::var("HOME") {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    let path = format!("{}/.toko.yaml", home);

    match args.command {
        Commands::Edit => {
            if let Err(e) = edit_snippets(path) {
                eprintln!("{}", e);
                exit(1);
            }
        }
        Commands::Search => {
            let snippets = load_snippets(path);

            if snippets.is_err() {
                eprintln!("failed to load snippets: {}", snippets.err().unwrap());
                exit(1);
            }

            let snippets_searcher = SnippetsSearcher::new(snippets.unwrap());
            let matched_snippets = snippets_searcher.search_blocking();

            for result in matched_snippets {
                println!("{}", result.cmd);
            }
        }
    }
}
