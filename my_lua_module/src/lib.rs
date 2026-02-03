use mlua_sys::lua54::lua::*;
use std::os::raw::c_int;
use log::LevelFilter;
use android_logger::Config;
mod magic_mount;
mod defs;
mod restorecon;
mod utils;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn _init() {}

unsafe extern "C-unwind" fn magic_mount(state: *mut lua_State) -> c_int { unsafe {
    //println!("Hello from Rust!");

    unsafe {
        let mount_source_cstr = lua_tolstring(state, 1, std::ptr::null_mut());
        
        if mount_source_cstr.is_null() {
            lua_pushstring(state, b"Error: missing or invalid mount_source parameter\0".as_ptr() as *const _);
            lua_error(state);
            return 0;
        }
        
        let mount_source = std::ffi::CStr::from_ptr(mount_source_cstr)
            .to_str()
            .unwrap_or("unknown");
        
        println!("Received mount_source from Lua: {}", mount_source);
        

        let _ = magic_mount::magic_mount(mount_source);
        
        lua_pushstring(state, b"Hello from Rust!\0".as_ptr() as *const _);
        1
    }

    
}}
// --------------------------
// Lua require("libmagic_mount") 时执行的入口
// --------------------------


#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libmagic_mount(state: *mut lua_State) -> c_int { unsafe {
    
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_tag("Magiskmount")
                .with_max_level(LevelFilter::Trace),
        );
        log::info!("logger initialized");
    });
    
    lua_newtable(state);

    lua_pushcfunction(state, magic_mount);
    lua_setfield(state, -2, b"magic_mount\0".as_ptr() as _);
    1
}}