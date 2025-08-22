use mlua::Lua;

/// Initializes the Lua state.
///
/// # Safety
///
/// This function should only be called once.
pub unsafe fn init(lua: &Lua) {
    lua.exec_raw((), |lstate| {
        super::init(lstate as *mut _);
    })
    .unwrap()
}
