use clap::Parser;

/// #[TODO]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Create challenge files in the current directory instead of a new one.
    #[arg(short, long)]
    pub in_place: bool,
}
