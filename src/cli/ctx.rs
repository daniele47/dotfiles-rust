use std::{collections::HashMap, env, str::FromStr};

use anyhow::Context;

use crate::{
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{AllProfiles, Profile},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paths {
    Home,
    Root,
    Backup,
    Config,
    Run,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliContext {
    paths: HashMap<Paths, AbsPathStr>,
    profiles: AllProfiles,
    all_name: RelPathStr,
}

impl CliContext {
    pub fn new(home: &Option<AbsPathStr>, root: &Option<AbsPathStr>) -> anyhow::Result<Self> {
        let paths = Self::load_paths(home, root)?;
        let profiles = Self::load_profiles(&paths[&Paths::Config])?;
        let all_name = RelPathStr::from_str("all")?;
        Ok(Self {
            paths,
            profiles,
            all_name,
        })
    }

    fn load_paths(
        home: &Option<AbsPathStr>,
        root: &Option<AbsPathStr>,
    ) -> anyhow::Result<HashMap<Paths, AbsPathStr>> {
        let mut paths = HashMap::new();

        // load home directory
        let home_dir;
        if let Some(home) = home {
            home_dir = home.clone();
        } else {
            let home = env::home_dir().context("Failure getting home directory")?;
            home_dir = AbsPathStr::new_from_pathbuf(home).context("Invalid home directory")?;
        }

        // load root directory
        let root_dir;
        if let Some(root) = root {
            root_dir = root.clone();
        } else {
            let root = env::current_dir().context("Failure getting root directory")?;
            root_dir = AbsPathStr::new_from_pathbuf(root).context("Invalid root directory")?;
        }

        // other dirs
        let backup_dir = root_dir.join(&RelPathStr::from_str("backup")?)?;
        let config_dir = root_dir.join(&RelPathStr::from_str("config")?)?;
        let run_dir = root_dir.join(&RelPathStr::from_str("run")?)?;

        paths.insert(Paths::Home, home_dir);
        paths.insert(Paths::Root, root_dir);
        paths.insert(Paths::Backup, backup_dir);
        paths.insert(Paths::Run, run_dir);
        paths.insert(Paths::Config, config_dir);

        Ok(paths)
    }

    fn load_profiles(config_dir: &AbsPathStr) -> anyhow::Result<AllProfiles> {
        let mut all_profiles = HashMap::new();

        // load nothing if there are no profiles
        if !config_dir.is_dir() {
            return Ok(AllProfiles::new(all_profiles));
        }

        // find and load all profiles config files
        config_dir.find(|ctx| {
            let fname = ctx.entry.file_name();
            let fname = fname.to_string_lossy();
            let conf_rel = ctx.path.to_rel(config_dir)?;
            let conf_str = conf_rel.to_string_lossy();

            // ignore dotfiles in config directory
            if fname.starts_with(".") {
                return Ok(false);
            }

            // normal profile parsing
            if let Some(pname) = conf_str.strip_suffix(".conf") {
                let profile = Profile::parse_config(&ctx.path.read_file()?, pname)?;
                all_profiles.insert(conf_rel, profile);
            }

            Ok(true)
        })?;

        Ok(AllProfiles::new(all_profiles))
    }

    pub fn path(&self, path: &Paths) -> &AbsPathStr {
        &self.paths[path]
    }

    pub fn profiles(&self) -> &AllProfiles {
        &self.profiles
    }

    pub fn all_name(&self) -> &RelPathStr {
        &self.all_name
    }
}
