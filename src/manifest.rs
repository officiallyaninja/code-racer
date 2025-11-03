use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
pub struct Manifest {
    data_version: u32,
    conversion: HashMap<String, Conversion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    langs: Vec<String>,
}

#[cfg(test)]
mod test {
    use color_eyre::eyre::Context;

    use crate::manifest::Manifest;

    #[test]
    fn basic_json() -> color_eyre::Result<()> {
        let json = r#"
    {
        "schema_version": 1,
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
