use anyhow::{Error, Result, bail};
use std::{
    fs::{create_dir_all},
    path::Path,
};

use crate::defs;
use std::fs::metadata;
#[allow(unused_imports)]
use std::fs::{Permissions, set_permissions};


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