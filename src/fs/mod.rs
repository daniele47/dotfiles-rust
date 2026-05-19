use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::Read,
    path::Component,
};

use anyhow::{Context, bail};
use tracing::{debug, instrument, trace};

use crate::fs::abs::AbsPathStr;

pub mod abs;
pub mod path;
pub mod rel;

#[derive(Debug, Default)]
pub struct FindCache {
    stack: Vec<(AbsPathStr, DirEntry)>,
}
impl FindCache {
    fn clear(&mut self) {
        self.stack.clear();
    }
}

impl AbsPathStr {
    fn list_raw(&self) -> anyhow::Result<ReadDir> {
        fs::read_dir(self.path()).with_context(|| {
            let p = self.display();
            format!("Could not list files in directory {p}")
        })
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn list<F>(&self, mut on_each: F) -> anyhow::Result<()>
    where
        F: FnMut(AbsPathStr, DirEntry) -> anyhow::Result<()>,
    {
        trace!(directory=%self.display(), "Finding files in directory:");
        self.list_raw()?.try_for_each(|e| {
            let e = e?;
            let abs = AbsPathStr::new_from_pathbuf(e.path())?;
            on_each(abs, e)
        })
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn find_with_cache<F>(&self, mut on_each: F, cache: &mut FindCache) -> anyhow::Result<()>
    where
        F: FnMut(AbsPathStr, DirEntry) -> anyhow::Result<()>,
    {
        cache.clear();
        let stack = &mut cache.stack;
        let mut root_traversed = false;
        let mut children;
        trace!(directory=%self.display(), "Finding files recursively in directory:");

        loop {
            let item = stack.pop();
            if !root_traversed {
                children = self.list_raw()?;
                root_traversed = true;
            } else if let Some((abs, dir_entry)) = item {
                children = abs.list_raw()?;
                on_each(abs, dir_entry)?;
            } else {
                break;
            }

            children.try_for_each(|dir_entry| {
                let dir_entry = dir_entry?;
                let abs = AbsPathStr::new_from_pathbuf(dir_entry.path())?;
                if dir_entry.file_type()?.is_dir() {
                    stack.push((abs, dir_entry));
                } else {
                    on_each(abs, dir_entry)?;
                }
                anyhow::Ok(())
            })?;
        }

        Ok(())
    }

    pub fn find<F>(&self, on_each: F) -> anyhow::Result<()>
    where
        F: FnMut(AbsPathStr, DirEntry) -> anyhow::Result<()>,
    {
        self.find_with_cache(on_each, &mut Default::default())
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn purge_path_opts(&self, allow_recursive_delete: bool) -> anyhow::Result<()> {
        // skip if path not exist
        if self.path().symlink_metadata().is_err() {
            trace!(path=%self.display(), "Path does not exist, nothing to delete:");
            return Ok(());
        }

        // purge symlink
        if self.path().symlink_metadata().is_ok_and(|f| f.is_symlink()) {
            fs::remove_file(self.path()).with_context(|| {
                let p = self.display();
                format!("Could not delete symlink: {p}")
            })?;
            debug!(symlink=%self.display(), "Symlink successfully deleted: ");
        }
        // purge file
        else if self.is_file() {
            fs::remove_file(self.path()).with_context(|| {
                let p = self.display();
                format!("Could not delete file: {p}")
            })?;
            debug!(file = %self.display(), "File successfully deleted: ");
        }
        // purge directory
        else if self.is_dir() {
            if allow_recursive_delete {
                fs::remove_dir_all(self.path()).with_context(|| {
                    let p = self.display();
                    format!("Could not delete directory recursively: {p}")
                })?;
                debug!(directory = %self.display(), "Directory successfully deleted recursively");
            } else {
                fs::remove_dir(self.path()).with_context(|| {
                    let p = self.display();
                    format!("Could not delete directory: {p}")
                })?;
                debug!(directory = %self.display(), "Directory successfully deleted");
            }
        }
        // fail if it was something else
        else {
            let p = self.display();
            bail!("Could not delete path: {p}");
        }

        // delete empty parent directories
        let mut parent = self.path().parent();
        while let Some(p) = parent {
            if fs::remove_dir(p).is_err() {
                break;
            }
            debug!(directory = %p.display(), "Deleted empty parent directory");
            parent = p.parent();
        }

        Ok(())
    }
    pub fn purge_path(&self) -> anyhow::Result<()> {
        self.purge_path_opts(false)
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn create_file(&self) -> anyhow::Result<()> {
        if self.is_file() {
            trace!(file = %self.display(), "File already exists, left untouched:");
            return Ok(());
        }

        // valid file can be created
        if !matches!(
            self.path().components().next_back(),
            Some(Component::Normal(_))
        ) {
            let p = self.display();
            bail!("Path cannot be created as a file: {p}")
        }

        // create parent dirs
        if let Some(parent) = self.path().parent() {
            fs::create_dir_all(parent).with_context(|| {
                let p = parent.display();
                format!("Failed to create directory: {p}")
            })?;
            debug!(directory = %parent.display(), "Parent directory successfully created:");
        } else {
            let p = self.display();
            bail!("Could not create parent directories: {p}");
        }

        // create file
        File::create(self.path()).with_context(|| {
            let p = self.display();
            format!("Failed to create file: {p}")
        })?;
        debug!(file = %self.display(), "File successfully created:");

        Ok(())
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn read_file(&self) -> anyhow::Result<String> {
        if !self.is_file() {
            let p = self.display();
            bail!("Cannot read a path that is not a file: {p}");
        }
        fs::read_to_string(self.path())
            .with_context(|| {
                let p = self.display();
                format!("Could not read file: {p}")
            })
            .inspect(|_| debug!(file = %self.display(), "File successfully read into string:"))
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display(), dst=%dst.display()))]
    pub fn copy_file(&self, dst: &Self) -> anyhow::Result<()> {
        dst.create_file()?;
        fs::copy(self.path(), dst.path()).with_context(|| {
            let p = self.display();
            let t = dst.display();
            format!("Failed to copy from {p} to {t}")
        })?;
        debug!(src_path = %self.display(), dst_path = %dst.display(), "Source file successfully copied into destination file:");
        Ok(())
    }

    #[instrument(ret, level = "trace", skip_all, fields(self = %self.display(), other = %other.display()))]
    pub fn files_eq(&self, other: &Self) -> bool {
        || -> anyhow::Result<()> {
            let sm = self.path().metadata()?;
            let om = other.path().metadata()?;

            // check both paths are files
            if !sm.is_file() || !om.is_file() {
                bail!("Not files");
            }

            // check file len for faster checks
            if sm.len() != om.len() {
                bail!("Length differs");
            }

            // chunked byte comparison (works for both text and binary)
            let mut file1 = File::open(self.path())?;
            let mut file2 = File::open(other.path())?;

            let mut buf1 = [0; 8192];
            let mut buf2 = [0; 8192];

            loop {
                let n1 = file1.read(&mut buf1)?;
                let n2 = file2.read(&mut buf2)?;

                if n1 != n2 || buf1[..n1] != buf2[..n2] {
                    bail!("Chunk differs");
                }
                if n1 == 0 {
                    return Ok(());
                }
            }
        }()
        .is_ok()
    }
}
