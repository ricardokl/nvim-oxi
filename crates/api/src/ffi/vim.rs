use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1146
    pub(crate) fn nvim_chan_send(
        chan: Integer,
        data: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L968
    pub(crate) fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L702
    pub(crate) fn nvim_del_current_line(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1549
    pub(crate) fn nvim_del_keymap(
        channel_id: u64,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L2000
    pub(crate) fn nvim_del_mark(
        name: NonOwning<String>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L746
    pub(crate) fn nvim_del_var(name: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L784
    pub(crate) fn nvim_echo(
        chunks: NonOwning<Array>,
        history: bool,
        opts: *const EchoOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L826
    pub(crate) fn nvim_err_write(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L837
    pub(crate) fn nvim_err_writeln(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L2114
    pub(crate) fn nvim_eval_statusline(
        str: NonOwning<String>,
        opts: *const EvalStatuslineOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L274
    pub(crate) fn nvim_feedkeys(
        keys: NonOwning<String>,
        mode: NonOwning<String>,
        escape_ks: bool,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1679
    pub(crate) fn nvim_get_chan_info(
        chan: Integer,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1275
    pub(crate) fn nvim_get_color_by_name(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1392
    pub(crate) fn nvim_get_color_map(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1411
    pub(crate) fn nvim_get_context(
        opts: *const GetContextOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L870
    pub(crate) fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L682
    pub(crate) fn nvim_get_current_line(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1183
    pub(crate) fn nvim_get_current_tabpage() -> TabHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L929
    pub(crate) fn nvim_get_current_win() -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L118
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_get_hl(
        ns_id: Integer,
        opts: *const GetHighlightOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L96
    pub(crate) fn nvim_get_hl_id_by_name(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L204
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_get_hl_ns(
        opts: *const GetNamespaceOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/vim.c#L1497
    pub(crate) fn nvim_get_keymap(
        mode: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1999
    pub(crate) fn nvim_get_mark(
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        opts: *const GetMarkOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1478
    pub(crate) fn nvim_get_mode(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1878
    pub(crate) fn nvim_get_proc(
        pid: Integer,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1838
    pub(crate) fn nvim_get_proc_children(
        pid: Integer,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L586
    pub(crate) fn nvim_get_runtime_file(
        name: NonOwning<String>,
        all: bool,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L714
    pub(crate) fn nvim_get_var(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L757
    pub(crate) fn nvim_get_vvar(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L360
    pub(crate) fn nvim_input(
        #[cfg(feature = "neovim-nightly")] channel_id: u64,
        keys: NonOwning<String>,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L390
    pub(crate) fn nvim_input_mouse(
        button: NonOwning<String>,
        action: NonOwning<String>,
        modifier: NonOwning<String>,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L849
    pub(crate) fn nvim_list_bufs(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1697
    pub(crate) fn nvim_list_chans() -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L557
    pub(crate) fn nvim_list_runtime_paths(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1162
    pub(crate) fn nvim_list_tabpages() -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1829
    pub(crate) fn nvim_list_uis(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L908
    pub(crate) fn nvim_list_wins(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1455
    pub(crate) fn nvim_load_context(dict: NonOwning<Dictionary>) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L527
    pub(crate) fn nvim_notify(
        msg: NonOwning<String>,
        log_level: Integer,
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and Nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1060
    pub(crate) fn nvim_open_term(
        buf: BufHandle,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        opts: *const OpenTermOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L816
    pub(crate) fn nvim_out_write(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1235
    pub(crate) fn nvim_paste(
        data: NonOwning<String>,
        crlf: bool,
        phase: Integer,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1302
    pub(crate) fn nvim_put(
        lines: NonOwning<Array>,
        r#type: NonOwning<String>,
        after: bool,
        follow: bool,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L474
    pub(crate) fn nvim_replace_termcodes(
        str: NonOwning<String>,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1924
    pub(crate) fn nvim_select_popupmenu_item(
        item: Integer,
        insert: bool,
        finish: bool,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        opts: *const SelectPopupMenuItemOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L880
    pub(crate) fn nvim_set_current_buf(buffer: BufHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L655
    pub(crate) fn nvim_set_current_dir(
        dir: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L692
    pub(crate) fn nvim_set_current_line(
        line: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1193
    pub(crate) fn nvim_set_current_tabpage(
        tabpage: TabHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L939
    pub(crate) fn nvim_set_current_win(window: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L172
    pub(crate) fn nvim_set_hl(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        channel_id: u64,
        ns_id: Integer,
        name: NonOwning<String>,
        val: *const SetHighlightOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L223
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_set_hl_ns(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L243
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_set_hl_ns_fast(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1537
    pub(crate) fn nvim_set_keymap(
        channel_id: u64,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        rhs: NonOwning<String>,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L736
    pub(crate) fn nvim_set_var(
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L768
    pub(crate) fn nvim_set_vvar(
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L544
    pub(crate) fn nvim_strwidth(
        text: NonOwning<String>,
        err: *mut Error,
    ) -> Integer;
}
