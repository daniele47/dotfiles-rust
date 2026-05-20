use std::env;

use anyhow::Context;
use autosaver::{cli::ctx::CliContext, fs::abs::AbsPathStr};

fn main() -> anyhow::Result<()> {
    let home = AbsPathStr::new_from_pathbuf(env::home_dir().context("err")?)?;
    let root = AbsPathStr::new_from_pathbuf(env::current_dir()?)?;
    let _ = CliContext::new(&Some(home), &Some(root))?;

    Ok(())
}
