use crate::types::capi_return_t;
use crate::types::tiledb_config_t;
use crate::types::tiledb_ctx_t;
use crate::types::tiledb_error_t;

extern "C" {
    pub fn tiledb_ctx_alloc(
        config: *mut tiledb_config_t,
        ctx: *mut *mut tiledb_ctx_t,
    ) -> capi_return_t;

    pub fn tiledb_ctx_free(ctx: *mut *mut tiledb_ctx_t);

    pub fn tiledb_ctx_get_stats(
        ctx: *mut tiledb_ctx_t,
        stats_json: *mut *mut ::std::os::raw::c_char,
    ) -> capi_return_t;

    pub fn tiledb_ctx_get_config(
        ctx: *mut tiledb_ctx_t,
        config: *mut *mut tiledb_config_t,
    ) -> capi_return_t;

    pub fn tiledb_ctx_get_last_error(
        ctx: *mut tiledb_ctx_t,
        err: *mut *mut tiledb_error_t,
    ) -> capi_return_t;

    pub fn tiledb_ctx_is_supported_fs(
        ctx: *mut tiledb_ctx_t,
        fs: u32,
        is_supported: *mut i32,
    ) -> capi_return_t;

    pub fn tiledb_ctx_cancel_tasks(ctx: *mut tiledb_ctx_t) -> capi_return_t;

    pub fn tiledb_ctx_set_tag(
        ctx: *mut tiledb_ctx_t,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> capi_return_t;
}
