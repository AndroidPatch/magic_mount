use anyhow::{Context, Error, Ok, Result, bail};
use std::{
    fs::{File, create_dir_all},
    io::{ErrorKind::AlreadyExists, Write},
    path::Path,
};

use crate::defs;
use std::fs::metadata;
#[allow(unused_imports)]
use std::fs::{Permissions, set_permissions};



pub fn ensure_clean_dir(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    log::debug!("ensure_clean_dir: {}", path.display());
    if path.exists() {
        log::debug!("ensure_clean_dir: {} exists, remove it", path.display());
        std::fs::remove_dir_all(path)?;
    }
    Ok(create_dir_all(path)?)
}

pub fn ensure_file_exists<T: AsRef<Path>>(file: T) -> Result<()> {
    match File::options().write(true).create_new(true).open(&file) {
        Result::Ok(_) => Ok(()),
        Err(err) => {
            if err.kind() == AlreadyExists && file.as_ref().is_file() {
                Ok(())
            } else {
                Err(Error::from(err))
                    .with_context(|| format!("{} is not a regular file", file.as_ref().display()))
            }
        }
    }
}

pub fn ensure_dir_exists<T: AsRef<Path>>(dir: T) -> Result<()> {
    let result = create_dir_all(&dir).map_err(Error::from);
    if dir.as_ref().is_dir() {
        result
    } else if result.is_ok() {
        bail!("{} is not a regular directory", dir.as_ref().display())
    } else {
        result
    }
}


pub fn get_tmp_path() -> &'static str {
    if metadata(defs::TEMP_DIR_LEGACY).is_ok() {
        return defs::TEMP_DIR_LEGACY;
    }
    if metadata(defs::TEMP_DIR).is_ok() {
        return defs::TEMP_DIR;
    }
    ""
}
pub fn get_work_dir() -> String {
    let tmp_path = get_tmp_path();
    format!("{}/workdir/", tmp_path)
}