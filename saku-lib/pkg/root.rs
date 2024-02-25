use std::fs;

use crate::prelude::*;
use crate::exec;
use crate::pkg::pkg::Pkg;
use crate::util::constants;
use crate::util::io;
use crate::util::msg;
use crate::util::{filepath, path};

impl Pkg {
    pub fn install_root(&self) -> Result<()> {
        self.store()?;
        self.link_root()?;
        Ok(())
    }
    pub fn store(&self) -> Result<()> {
        trace!("storing files");
        let has_artifacts = !io::mkdir(path::store_dir(&self.name))?;
        if has_artifacts {
            debug!("cleaning up artifacts in store");
            let files = path::get_stored_bin(&self.name)?;
            for entry in files {
                debug!("removing artifact {}", msg::general::path_f(&entry));
                exec::unlink(&entry)?;
            }
            let store_path = path::store_dir(&self.name);
            let dirs = path::get_artifact_dirs(&self.name)?;
            for entry in dirs {
                debug!("removing artifact {}",  msg::general::path_f(&entry));
                io::rmdir(&entry)?;
            }
            io::mkdir(store_path)?;
        }
        exec::install(&self.name, &self.group)?;
        Ok(())
    }
    pub fn link_root(&self) -> Result<()> {
        trace!("linking root");
        let files = path::get_stored_files(&self.name)?;
        debug!("{:?}", files);
        for entry in &files {
            if filepath::is_dir(&entry) {
                debug!("skipping dir {entry}");
                continue;
            }
            self.link_entry(entry)?;
        }
        Ok(())
    }
    pub fn link_entry(&self, path: &str) -> Result<()> {
        let rel = filepath::get_relative(&path::store_dir(&self.name), path)?;
        debug!("found {}", rel);
        let root_path = filepath::join(&*constants::ROOT_DIR, &rel);
        if filepath::exists(&root_path) {
            debug!("root file already exists {root_path}. cleaning up");
            std::fs::remove_file(&root_path)?;
        }
        io::mkdir(filepath::parent_dir(&root_path)?)?;
        msg::link(&path, &root_path);
        io::link(&path, &root_path)?;
        Ok(())
    }
    pub fn uninstall_root(&self) -> Result<()> {
        for d in fs::read_dir(path::root_dir(&format!("{}/bin", self.name)))? {
            let d = d?;
            let d_path_bind = d.path();
            let d_path = match d_path_bind.to_str() {
                Some(s) => Ok(s),
                None => Err(Error::Unexpected),
            }?;
            if filepath::is_dir(d_path) {
                continue;
            }
            let name = filepath::base_name(d_path)?;
            fs::remove_file(d_path)?;
            fs::remove_file(path::root_file("bin", &name))?;
        }
        Ok(())
    }
}
