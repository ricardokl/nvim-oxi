use types::{self as nvim, Array, Object, conversion::FromObject};

use crate::Result;
use crate::choose;
use crate::ffi::vimscript::*;
use crate::opts::ExecOpts;
use crate::types::*;

/// Binding to [`nvim_call_dict_function()`][1].
///
/// Calls a VimL dictionary function with the given arguments, returning the
/// result of the funtion call.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_call_dict_function()
pub fn call_dict_function<Args, Ret>(
    dict: &str,
    func: &str,
    args: Args,
) -> Result<Ret>
where
    Args: Into<Array>,
    Ret: FromObject,
{
    let dict = Object::from(nvim::String::from(dict));
    let func = nvim::String::from(func);
    let args = args.into();
    let mut err = nvim::Error::new();
    let res = unsafe {
        nvim_call_dict_function(
            dict.non_owning(),
            func.as_nvim_str(),
            args.non_owning(),
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(Ret::from_object(res)?))
}

/// Binding to [`nvim_call_function()`][1].
///
/// Calls a VimL function with the given arguments, returning the result of the
/// funtion call.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_call_function()
pub fn call_function<Args, Ret>(func: &str, args: Args) -> Result<Ret>
where
    Args: Into<Array>,
    Ret: FromObject,
{
    let func = nvim::String::from(func);
    let args = args.into();
    let mut err = nvim::Error::new();
    let res = unsafe {
        nvim_call_function(
            func.as_nvim_str(),
            args.non_owning(),
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(Ret::from_object(res)?))
}

/// Binding to [`nvim_command()`][1].
///
/// Executes an Ex command.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_command()
pub fn command(command: &str) -> Result<()> {
    let command = nvim::String::from(command);
    let mut err = nvim::Error::new();
    unsafe { nvim_command(command.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_eval()`][1].
///
/// Evaluates a VimL expression.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_eval()
pub fn eval<V>(expr: &str) -> Result<V>
where
    V: FromObject,
{
    let expr = nvim::String::from(expr);
    let mut err = nvim::Error::new();
    let output =
        unsafe { nvim_eval(expr.as_nvim_str(), types::arena(), &mut err) };
    choose!(err, Ok(V::from_object(output)?))
}

/// Binding to [`nvim_exec2()`][1].
///
/// Executes Vimscript (multiline block of Ex commands), like anonymous
/// `:source`.
///
/// Unlike [`command`] this function supports heredocs, script-scope (s:), etc.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_exec2()
pub fn exec2(src: &str, opts: &ExecOpts) -> Result<Option<nvim::String>> {
    let src = types::String::from(src);
    let mut err = types::Error::new();
    let dict = unsafe {
        nvim_exec2(crate::LUA_INTERNAL_CALL, src.as_nvim_str(), opts, &mut err)
    };
    choose!(err, {
        Ok(dict
            .into_iter()
            .next()
            .map(|(_s, output)| nvim::String::from_object(output).unwrap()))
    })
}

/// Binding to [`nvim_parse_expression()`][1].
///
/// Parses a VimL expression.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_parse_expression()
pub fn parse_expression(
    expr: &str,
    flags: &str,
    include_highlight: bool,
) -> Result<ParsedVimLExpression> {
    let expr = nvim::String::from(expr);
    let flags = nvim::String::from(flags);
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_parse_expression(
            expr.as_nvim_str(),
            flags.as_nvim_str(),
            include_highlight,
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(ParsedVimLExpression::from_object(dict.into())?))
}
