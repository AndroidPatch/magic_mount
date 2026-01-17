
use const_format::concatcp;
pub const ADB_DIR: &str = "/data/adb/";

pub const AP_OVERLAY_SOURCE: &str = "APatch";
pub const MODULE_DIR: &str = concatcp!(ADB_DIR, "modules/");

pub const TEMP_DIR: &str = "/debug_ramdisk";
pub const TEMP_DIR_LEGACY: &str = "/sbin";


pub const DISABLE_FILE_NAME: &str = "disable";
pub const SKIP_MOUNT_FILE_NAME: &str = "skip_mount";



