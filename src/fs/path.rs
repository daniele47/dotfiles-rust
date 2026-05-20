use std::{
    borrow::Cow,
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, bail};
use internment::Intern;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathStr {
    path: Intern<PathBuf>,
}

impl PathStr {
    pub fn new_from_pathbuf(path: PathBuf) -> anyhow::Result<Self> {
        // check path contains invalid components
        if !Intern::<PathBuf>::is_interned(&path) {
            for component in path.components() {
                if component == Component::ParentDir {
                    bail!("Path contains parent directory: {}", path.display());
                } else if component == Component::CurDir {
                    bail!("Path contains current directory: {}", path.display());
                }
            }
        }
        Ok(Self {
            path: Intern::new(path),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn new(path: String) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(path.into())
    }

    pub fn to_str(&self) -> Option<&str> {
        self.path().to_str()
    }

    pub fn to_string_lossy<'a>(&'a self) -> Cow<'a, str> {
        self.path.to_string_lossy()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }

    pub fn basename(&self) -> anyhow::Result<Self> {
        self.path()
            .file_name()
            .map(|f| Self::new_from_pathbuf(PathBuf::from(f)))
            .with_context(|| format!("Could not get basename of {}", self.display()))?
    }

    pub fn same_path(&self, other: &Self) -> bool {
        self.path().components() == other.path().components()
    }
}

impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
