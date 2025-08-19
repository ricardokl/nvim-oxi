use core::ffi::c_int;
use std::panic;

use luajit::{Pushable, ffi::State};

use crate::Error;

/// The entrypoint of the plugin.
///
/// Initializes the Lua state, executes the entrypoint function and pushes the
/// result on the stack.
#[inline(always)]
pub unsafe fn entrypoint<T>(lua_state: *mut State, body: fn() -> T) -> c_int
where
    T: Pushable,
{
    unsafe {
        // Initialize arena, luajit, and libuv in sequence using and_then
        let init_result = types::arena_init()
            .map_err(Error::from)
            .and_then(|_| luajit::init(lua_state).map_err(Error::from))
            .and_then(|_| {
                #[cfg(feature = "libuv")]
                {
                    libuv::init(lua_state).map_err(Error::from)
                }
                #[cfg(not(feature = "libuv"))]
                {
                    Ok(())
                }
            });

        // Execute body and push result
        let result: std::result::Result<c_int, Error> = match init_result {
            Ok(()) => {
                // Catch panics from body()
                match panic::catch_unwind(panic::AssertUnwindSafe(body)) {
                    Ok(result) => match result.push(lua_state) {
                        Ok(count) => Ok(count),
                        Err(e) => Err(Error::from(e)),
                    },
                    Err(panic_payload) => {
                        // Convert panic to RuntimeError
                        let panic_msg = if let Some(s) =
                            panic_payload.downcast_ref::<&str>()
                        {
                            s.to_string()
                        } else if let Some(s) =
                            panic_payload.downcast_ref::<String>()
                        {
                            s.clone()
                        } else {
                            "Unknown panic occurred".to_string()
                        };

                        let runtime_error = luajit::Error::RuntimeError(
                            format!("Plugin panic: {}", panic_msg),
                        );
                        Err(Error::from(runtime_error))
                    },
                }
            },
            Err(e) => Err(e),
        };

        match result {
            Ok(num_pushed) => num_pushed,
            Err(err) => luajit::utils::push_error(&err, lua_state),
        }
    }
}
