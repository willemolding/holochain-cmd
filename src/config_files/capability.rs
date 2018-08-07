use error::DefaultResult;
use serde_json;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// ### JSON
/// ```json
/// {
///   "build": {
///     "artifact": "target/release/some_name.wasm",
///     "build": [ "cargo build" ],
///     "test": [ "cargo test" ]
///   }
/// }
/// ```
///
/// ### TOML
/// ```toml
/// [build]
/// artifact = "target/release/some_name.wasm"
/// build = [ "cargo build" ]
/// test = [ "cargo test" ]
/// ```
///
/// ### YAML
/// ```yaml
/// build:
///   artifact: "target/release/some_name.wasm"
///   build:
///     - cargo build
///   test:
///     - cargo test
/// ```
#[derive(Serialize, Deserialize)]
pub struct Capability {
    build: BuildSection,
}

#[derive(Serialize, Deserialize)]
pub struct BuildSection {
    artifact: PathBuf,
    build: Vec<String>,
    test: Vec<String>,
}

impl Capability {
    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<Capability> {
        let file = File::open(&path)?;

        let zome = serde_json::from_reader(file)?;

        Ok(zome)
    }

    pub fn save_as<T: AsRef<Path>>(&self, path: T) -> DefaultResult<()> {
        let file = File::create(&path)?;

        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }
}
