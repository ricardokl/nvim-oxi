//! Tests for the `FromObject` trait.

use nvim_oxi::{
    api,
    conversion::{self, FromObject, ToObject},
    lua,
    Object,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestStruct {
    foo: i32,
    bar: String,
}

impl FromObject for TestStruct {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(nvim_oxi::serde::Deserializer::new(obj))
            .map_err(Into::into)
    }
}

impl ToObject for TestStruct {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(nvim_oxi::serde::Serializer::new())
            .map_err(Into::into)
    }
}

impl lua::Poppable for TestStruct {
    unsafe fn pop(
        lstate: *mut lua::ffi::State,
    ) -> Result<Self, lua::Error> {
        unsafe {
            let obj = Object::pop(lstate)?;
            Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
        }
    }
}

impl lua::Pushable for TestStruct {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        unsafe {
            self.to_object()
                .map_err(lua::Error::push_error_from_err::<Self, _>)?
                .push(lstate)
        }
    }
}

#[nvim_oxi::test]
fn test_struct_from_valid_lua_table() {
    let key = "test_struct_from_valid_lua_table";
    api::set_var(key, TestStruct { foo: 42, bar: "baz".into() }).unwrap();

    let lua_code = r#"
        local ts = vim.g.test_struct_from_valid_lua_table
        assert(ts.foo == 42)
        assert(ts.bar == "baz")
    "#;

    api::command(&format!(":lua {}", lua_code)).unwrap();
}

#[nvim_oxi::test]
#[should_panic]
fn test_struct_from_invalid_lua_table() {
    let key = "test_struct_from_invalid_lua_table";
    api::set_var(key, TestStruct { foo: 42, bar: "baz".into() }).unwrap();

    let lua_code = r#"
        local ts = vim.g.test_struct_from_invalid_lua_table
        ts.fooo = ts.foo
        ts.foo = nil
        vim.g.test_struct_from_invalid_lua_table = ts
    "#;

    api::command(&format!(":lua {}", lua_code)).unwrap();

    let _: TestStruct = api::get_var(key).unwrap();
}
