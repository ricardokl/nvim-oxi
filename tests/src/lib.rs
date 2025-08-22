// Suppress warnings when testing deprecated functions.
#![allow(deprecated)]

mod api;
mod from_object;
mod r#macro;

// Libuv bindings don't work on Windows.
#[cfg(not(any(target_os = "windows", target_env = "msvc")))]
mod libuv;
