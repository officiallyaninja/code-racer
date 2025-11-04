use code_racer::{
    manifest::{Completion, Conversion, Manifest},
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
        let challenge_category =
            entry.unwrap_or_else(|e| panic!("error reading directory challenges/ : {e}"));
        let challenge_category_path = challenge_category.path();
        let challenge_category_name = challenge_category
            .file_name()
            .into_string()
            .expect("failed to convert OSString to String");

        dbg!(&challenge_category_name);

        for entry in PathBuf::from(&challenge_category_path)
            .read_dir()
            .expect("we got this path fom a dir entry so it should not error")
        {
            let challenge = entry.unwrap_or_else(|e| {
                panic!(
                    "error reading directory challenges/{}/ : {e}",
                    challenge_category_path.display()
                )
            });

            dbg!(&challenge.file_name().into_string().unwrap());

            match challenge_category_name.as_ref() {
                "completion" => {
                    let name = challenge
                        .file_name()
                        .into_string()
                        .expect("failed to convert OSString to String");
                    let mut langs = Vec::new();
                    for lang_folder in PathBuf::from(challenge.path())
                        .read_dir()
                        .expect("we got this path fom a dir entry so it should not error")
                    {
                        let lang_folder = lang_folder.unwrap_or_else(|e| {
                            panic!("error reading challenges/completion/'{}': {e}", name)
                        });
                        langs.push(
                            lang_folder
                                .file_name()
                                .into_string()
                                .expect("failed to convert OSString to String"),
                        );
                    }
                    completion_challenges
                        .insert(name.clone(), Completion { langs })
                        .map(|_| panic!("duplicate {}, {:?}", name, completion_challenges));
                }
                "conversion" => {
                    let name = challenge
                        .file_name()
                        .into_string()
                        .expect("failed to convert OSString to String");
                    let mut langs = Vec::new();
                    for lang_file in PathBuf::from(challenge.path())
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
                _ => {
                    panic!("Unexpected directory")
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
