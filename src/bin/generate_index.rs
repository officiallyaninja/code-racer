use code_racer::{
    manifest::{ChallengeType, Completion, Conversion, Manifest},
    DATA_VERSION,
};
use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf};

fn main() {
    // Verify this works in CI
    let project_root = env::var("PROJECT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    dbg!(&project_root);
    let challenges_dir = project_root.join("challenges");

    let mut completion_challenges: HashMap<String, Completion> = HashMap::new();
    let mut conversion_challenges: HashMap<String, Conversion> = HashMap::new();

    for entry in challenges_dir
        .read_dir()
        .expect("Error reading directory challenges/")
    {
        let challenge_type_dir =
            entry.unwrap_or_else(|e| panic!("error reading directory challenges/ : {e}"));
        let challenge_type_path = challenge_type_dir.path();
        let challenge_type_name = challenge_type_dir
            .file_name()
            .into_string()
            .expect("failed to convert OSString to String");

        for entry in PathBuf::from(&challenge_type_path)
            .read_dir()
            .expect("we got this path fom a dir entry so it should not error")
        {
            let challenge_dir = entry.unwrap_or_else(|e| {
                panic!(
                    "error reading directory challenges/{}/ : {e}",
                    challenge_type_path.display()
                )
            });

            let challenge_type = ChallengeType::try_from(challenge_type_name.as_ref())
                .expect("Unexpected directory");
            match challenge_type {
                ChallengeType::Completion => {
                    let challenge_name = challenge_dir
                        .file_name()
                        .into_string()
                        .expect("failed to convert OSString to String");

                    completion_challenges
                        .insert(
                            challenge_name.clone(),
                            Completion::new_from_fs(&PathBuf::from(challenge_dir.path())),
                        )
                        .map(|_| {
                            panic!("duplicate {}, {:?}", challenge_name, completion_challenges)
                        });
                }
                ChallengeType::Conversion => {
                    let name = challenge_dir
                        .file_name()
                        .into_string()
                        .expect("failed to convert OSString to String");
                    let mut langs = Vec::new();
                    for lang_file in PathBuf::from(challenge_dir.path())
                        .read_dir()
                        .expect("we got this path fom a dir entry so it should not error")
                    {
                        let lang_folder = lang_file.unwrap_or_else(|e| {
                            panic!("error reading challenges/completion/'{}': {e}", name)
                        });
                        langs.push(
                            lang_folder
                                .file_name()
                                .into_string()
                                .expect("failed to convert OSString to String")
                                .split_once('.')
                                .expect("file should contain a '.' ")
                                .0
                                .to_string(),
                        );
                    }
                    conversion_challenges
                        .insert(name.clone(), Conversion { langs })
                        .map(|_| panic!("duplicate {}, {:?}", name, conversion_challenges));
                }
            }
        }
    }
    let manifest = Manifest {
        data_version: DATA_VERSION,
        conversion: conversion_challenges,
        completion: completion_challenges,
    };

    let manifest_json =
        serde_json::to_string_pretty(&manifest).expect("serialization should not fail");
    let mut manifest_file =
        File::create(project_root.join("manifest.json")).expect("file failed to be created");
    manifest_file
        .write_all(manifest_json.as_bytes())
        .expect("failed to write to manifest.json");
}
