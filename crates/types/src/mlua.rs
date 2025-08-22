use mlua::{Lua, UserData};

use super::arena;

struct ArenaHandle;

impl UserData for ArenaHandle {}

/// Initializes the arena and ties its lifetime to the Lua state.
pub fn init_arena(lua: &Lua) -> mlua::Result<()> {
    arena::arena_init();
    // The arena is stored in a thread-local static, and there's no way to
    // de-initialize it. To tie its lifetime to the Lua state we create a dummy
    // userdata and store it in the registry. When the Lua state is closed, the
    // userdata will be garbage collected. We don't need to do anything on drop.
    lua.set_named_registry_value("nvim-oxi-arena", ArenaHandle)
}
