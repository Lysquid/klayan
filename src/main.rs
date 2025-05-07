use std::{fs::File, io::BufReader, path::PathBuf, process};

use clap::Parser;
use env_logger;
use klayan::analyse;
use klayan::kalamine::{Corpus, Layout};

/// Analyse a keyboard layout
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Layout to analyse, in json format
    layout: PathBuf,
    /// Corpus to use for analysis, in json format
    corpus: PathBuf,
}

fn main() {
    let mut log_builder = env_logger::Builder::new();
    if cfg!(debug_assertions) {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Warn);
    }
    log_builder.init();

    let cli = Cli::parse();

    let layout = File::open(cli.layout).unwrap_or_else(|err| {
        eprintln!("Could not open layout file: {err}");
        process::exit(1);
    });

    let corpus = File::open(cli.corpus).unwrap_or_else(|err| {
        eprintln!("Could not open corpus file: {err}");
        process::exit(1);
    });

    let layout: Layout = serde_json::from_reader(BufReader::new(layout)).unwrap_or_else(|err| {
        eprint!("Invalid layout file: {err}");
        process::exit(1);
    });

    let corpus: Corpus = serde_json::from_reader(BufReader::new(corpus)).unwrap_or_else(|err| {
        eprintln!("Invalid corpus file: {err}");
        process::exit(1);
    });

    analyse(&layout, &corpus);
}
