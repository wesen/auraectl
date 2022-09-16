use config::{Config, ConfigError, File, FileFormat, FileSourceFile};
use home::home_dir;
use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Pki {
    root: PathBuf,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    verbose: bool,
    pki: Pki,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::builder();

        if let Some(mut path) = home_dir() {
            path.push(".aurae");
            let source: File<FileSourceFile, FileFormat> = path.into();
            s = s.add_source(source);
        }

        let s = s.build()?;
        s.try_deserialize()
    }
}
