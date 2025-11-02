use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use clap::Parser;

const PATH: &str = "code-race";
const MARKER_EXT: &str = ".racer";
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Create challenge files in the current directory instead of a new one.
    #[arg(short, long)]
    in_place: bool,
}

fn create_marker_file(dir: &Path) -> anyhow::Result<()> {
    fs::create_dir(dir).context("could not create .racer marker file")?;
    todo!("add metadata to .racer file")
}

fn is_valid_challenge_folder(dir: &PathBuf) -> anyhow::Result<()> {
    let x = dir
        .read_dir()
        .context(format!("could not read directory: {}", dir.display()))?
        .map(|e| {
            todo!("check if .racer file exists (and figure out when to err and with what message)")
        });
    todo!("verify if .racer has expected data")
}

fn main() -> anyhow::Result<()> {
    let Args { in_place } = Args::parse();

    // TODO: fetch the actual data about the challenge
    let cur_dir = std::env::current_dir().context("Could not get current directory")?;
    let challenge_dir = if in_place {
        if cur_dir
            .read_dir()
            .context(format!("Failed to read directory: {}", cur_dir.display()))?
            .next()
            .is_some()
        {
            anyhow::bail!("Current directory is not empty, run without --in-place to create a new folder for challenge");
        } else {
            create_marker_file(&cur_dir)?;
        }
        cur_dir
    // not in place
    } else {
        let path = cur_dir.join(PATH);
        if path.exists() {
            is_valid_challenge_folder(&path)?;
        } else {
            fs::create_dir(PATH).context(format!("could not create folder '{PATH}'"))?;
            create_marker_file(&path);
        }
        path
    };

    Ok(())
}
