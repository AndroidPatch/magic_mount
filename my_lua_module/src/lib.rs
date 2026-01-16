use mlua_sys::lua54::lua::*;
use std::os::raw::c_int;
use log::LevelFilter;
use android_logger::Config;
mod magic_mount;
mod defs;
mod restorecon;
mod utils;


unsafe extern "C-unwind" fn magic_mount(state: *mut lua_State) -> c_int { unsafe {
    //println!("Hello from Rust!");




    magic_mount::magic_mount();
    lua_pushstring(state, b"Hello from Rust!\0".as_ptr() as *const _);
    1
}}
// --------------------------
// Lua require("libmagic_mount") 时执行的入口
// --------------------------


#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libmagic_mount(L: *mut lua_State) -> c_int { unsafe {
    
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_tag("Magiskmount")
                .with_max_level(LevelFilter::Trace),
        );
        log::info!("logger initialized");
    });
    
    lua_newtable(L);

    lua_pushcfunction(L, magic_mount);
    lua_setfield(L, -2, b"magic_mount\0".as_ptr() as _);
    1
}}