mod args;

use clap::Parser;
use color_eyre::eyre::{bail, Context};
use std::{fs, path::Path};

const PATH: &str = "code-race";
const MARKER_EXT: &str = ".racer";

fn create_marker_file(dir: &Path) -> color_eyre::Result<()> {
    fs::create_dir(dir).context("could not create .racer marker file")?;
    // TODO: add metadata to .racer file
    Ok(())
}

/// Checks if the directory is a valid challenge folder.
/// That is checks if it has a .racer file.
///
/// TODO: make it check if the files in dir match data in the .racer file
///
/// This is to ensure that we don't accidentally modify a directory the user is using.
fn validate_challenge_folder(dir: &Path) -> color_eyre::Result<()> {
    if !dir.join(MARKER_EXT).exists() {
        bail!("Missing .racer marker file");
    }
    // TODO: use SERDE to read file and check if the data in .racer matches the folder
    let dir_entries = dir
        .read_dir()
        .context(format!("could not read directory: {}", dir.display()))?
        .map(|e| match e {
            Ok(entry) => entry,
            Err(error) => todo!("I'm not sure how to handle these tbh"),
        })
        .collect::<Vec<_>>();
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    let args = args::Args::parse();

    // TODO: fetch the actual data about the challenge
    let cur_dir = std::env::current_dir().context("Could not get current directory")?;
    let challenge_dir = if args.in_place {
        if cur_dir
            .read_dir()
            .context(format!("Failed to read directory: {}", cur_dir.display()))?
            .next()
            .is_some()
        {
            bail!("Current directory is not empty, run without --in-place to create a new folder for challenge");
        } else {
            create_marker_file(&cur_dir)?;
        }
        cur_dir
    // not in place
    } else {
        let path = cur_dir.join(PATH);
        if path.exists() {
            validate_challenge_folder(&path)?;
        } else {
            fs::create_dir(PATH).context(format!("could not create folder '{PATH}'"))?;
            create_marker_file(&path)?;
        }
        path
    };

    Ok(())
}

#[cfg(test)]
mod test {

    use color_eyre::eyre::Context;

    use crate::{create_marker_file, validate_challenge_folder, MARKER_EXT};

    #[test]
    fn make_simple() -> color_eyre::Result<()> {
        color_eyre::install()?;
        let temp = tempfile::tempdir().context("Failed to create Temp file")?;
        create_marker_file(temp.path()).context("Failed to create Marker file")?;

        assert!(
            temp.path().join(MARKER_EXT).exists(),
            "{MARKER_EXT} file missing",
        );

        validate_challenge_folder(temp.path())?;
        Ok(())
    }
}
