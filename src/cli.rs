use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// An arbitrary-precision calculator.
#[derive(Debug, Parser)]
#[command(name = "dcrs")]
#[command(about = "An arbitrary-precision calculator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    File { file: Option<PathBuf> },
    #[command(arg_required_else_help = true)]
    Expression { expression: Option<String> },
}
