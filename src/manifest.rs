use macros::generate_challenge_types;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

//schema version: int
//time generated: timestamp
//challenges:
//  {
//  conversions: [
//      name: String
//      langs: Vec<String>
//      difficulty: ?
//  ]
//  -- other
//  }
//

#[derive(Serialize, Deserialize, Debug)]
#[generate_challenge_types]
pub struct Manifest {
    pub data_version: u32,
    pub conversion: HashMap<String, Conversion>,
    pub completion: HashMap<String, Completion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub langs: Vec<String>,
}

impl Completion {
    pub fn new_from_fs(path: &Path) -> Self {
        let mut langs = Vec::new();
        for lang_folder in path
            .read_dir()
            .expect("we got this path fom a dir entry so it should not error")
        {
            let lang_folder = lang_folder.unwrap_or_else(|e| {
                panic!(
                    "error reading challenges/completion/'{}': {e}",
                    path.file_name()
                        .expect("path should have a file_name")
                        .to_str()
                        .expect("&Osstr should convert to str")
                )
            });
            langs.push(
                lang_folder
                    .file_name()
                    .into_string()
                    .expect("failed to convert OSString to String"),
            );
        }
        Self { langs }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub langs: Vec<String>,
}

// pub struct Conversion {
//     pub langs: Vec<String>,
// }

#[cfg(test)]
mod test {
    use color_eyre::eyre::Context;

    use crate::manifest::Manifest;

    #[test]
    fn basic_json() -> color_eyre::Result<()> {
        let json = r#"
    {
        "data_version": 1,
        "conversion": {
            "fizz-buzz": { "langs": ["rust", "go"] },
            "palindrome": { "langs": ["python", "javascript"] }
        }
    }"#;

        let manifest: Manifest =
            serde_json::from_str(json).context("failed to create Manifest struct")?;
        println!("{:#?}", manifest);

        assert_eq!(manifest.data_version, 1);
        assert_eq!(manifest.conversion["fizz-buzz"].langs, vec!["rust", "go"]);

        // Serialize back
        let out = serde_json::to_string_pretty(&manifest)
            .context("failed to convert manifest struct to json")?;
        println!("{}", out);

        Ok(())
    }
}
