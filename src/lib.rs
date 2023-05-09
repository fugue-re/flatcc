use std::env;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub use flatc_rust::{Args, Error, Version};

use flatc_rust::Flatc;

#[repr(transparent)]
pub struct Builder(Flatc);

impl Builder {
    pub fn new() -> Self {
        Self(Flatc::from_path(if matches!(env::var("HOST"), Ok(host) if host.contains("windows")) {
            PathBuf::from_iter([env!("OUT_DIR"), "bin", "flatc.exe"])
        } else {
            PathBuf::from_iter([env!("OUT_DIR"), "bin", "flatc"])
        }))
    }
}

impl Deref for Builder {
    type Target = Flatc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn build(path: impl AsRef<Path>) -> Result<(), Error> {
    Builder::new().run(Args {
        inputs: &[path.as_ref()],
        out_dir: Path::new(&env::var("OUT_DIR").unwrap()),
        ..Default::default()
    })
}

pub fn build_all<'a>(paths: impl IntoIterator<Item=&'a Path>) -> Result<(), Error> {
    Builder::new().run(Args {
        inputs: &paths.into_iter().collect::<Vec<_>>(),
        out_dir: Path::new(&env::var("OUT_DIR").unwrap()),
        ..Default::default()
    })
}
