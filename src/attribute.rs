extern crate tiledb_sys as ffi;

pub use tiledb_sys::Datatype;
pub use tiledb_sys::FilterType;

use crate::context::Context;
use crate::error::Error;
use crate::filter_list::FilterList;
use crate::Result as TileDBResult;

pub struct Attribute {
    _wrapped: *mut ffi::tiledb_attribute_t,
}

impl Attribute {
    pub fn new(
        ctx: &Context,
        name: &str,
        datatype: Datatype,
    ) -> TileDBResult<Attribute> {
        let mut attr = Attribute {
            _wrapped: std::ptr::null_mut::<ffi::tiledb_attribute_t>(),
        };
        let c_name =
            std::ffi::CString::new(name).expect("Error creating CString");
        let res = unsafe {
            ffi::tiledb_attribute_alloc(
                ctx.as_mut_ptr(),
                c_name.as_c_str().as_ptr(),
                datatype as u32,
                &mut attr._wrapped,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(attr)
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn name(&self, ctx: &Context) -> TileDBResult<String> {
        let mut c_name = std::ptr::null::<std::ffi::c_char>();
        let res = unsafe {
            ffi::tiledb_attribute_get_name(
                ctx.as_mut_ptr(),
                self._wrapped,
                &mut c_name,
            )
        };
        if res == ffi::TILEDB_OK {
            let c_name = unsafe { std::ffi::CStr::from_ptr(c_name) };
            Ok(String::from(c_name.to_string_lossy()))
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn datatype(&self, ctx: &Context) -> TileDBResult<Datatype> {
        let mut c_dtype: std::ffi::c_uint = 0;
        let res = unsafe {
            ffi::tiledb_attribute_get_type(
                ctx.as_mut_ptr(),
                self._wrapped,
                &mut c_dtype,
            )
        };
        if res == ffi::TILEDB_OK {
            if let Some(dtype) = Datatype::from_u32(c_dtype) {
                Ok(dtype)
            } else {
                Err(Error::from("Invalid Datatype value returned by TileDB"))
            }
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn set_nullable(
        &self,
        ctx: &Context,
        nullable: bool,
    ) -> TileDBResult<()> {
        let nullable: u8 = if nullable { 1 } else { 0 };
        let res = unsafe {
            ffi::tiledb_attribute_set_nullable(
                ctx.as_mut_ptr(),
                self._wrapped,
                nullable,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(())
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn get_nullable(&self, ctx: &Context) -> TileDBResult<bool> {
        let mut c_nullable: std::ffi::c_uchar = 0;
        let res = unsafe {
            ffi::tiledb_attribute_get_nullable(
                ctx.as_mut_ptr(),
                self._wrapped,
                &mut c_nullable,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(c_nullable == 1)
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn set_filter_list(
        &self,
        ctx: &Context,
        filter_list: &FilterList,
    ) -> TileDBResult<()> {
        let res = unsafe {
            ffi::tiledb_attribute_set_filter_list(
                ctx.as_mut_ptr(),
                self._wrapped,
                filter_list.as_mut_ptr(),
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(())
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn get_filter_list(&self, ctx: &Context) -> TileDBResult<FilterList> {
        let mut flist = FilterList::default();
        let res = unsafe {
            ffi::tiledb_attribute_get_filter_list(
                ctx.as_mut_ptr(),
                self._wrapped,
                flist.as_mut_ptr_ptr(),
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(flist)
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn set_var_sized(&self, ctx: &Context) -> TileDBResult<()> {
        self.set_cell_val_num(ctx, u32::MAX)
    }

    pub fn is_var_sized(&self, ctx: &Context) -> TileDBResult<bool> {
        self.get_cell_val_num(ctx).map(|num| num == u32::MAX)
    }

    pub fn set_cell_val_num(
        &self,
        ctx: &Context,
        num: u32,
    ) -> TileDBResult<()> {
        let c_num = num as std::ffi::c_uint;
        let res = unsafe {
            ffi::tiledb_attribute_set_cell_val_num(
                ctx.as_mut_ptr(),
                self._wrapped,
                c_num,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(())
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn get_cell_val_num(&self, ctx: &Context) -> TileDBResult<u32> {
        let mut c_num: std::ffi::c_uint = 0;
        let res = unsafe {
            ffi::tiledb_attribute_get_cell_val_num(
                ctx.as_mut_ptr(),
                self._wrapped,
                &mut c_num,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(c_num as u32)
        } else {
            Err(ctx.expect_last_error())
        }
    }

    pub fn get_cell_size(&self, ctx: &Context) -> TileDBResult<u64> {
        let mut c_size: std::ffi::c_ulonglong = 0;
        let res = unsafe {
            ffi::tiledb_attribute_get_cell_size(
                ctx.as_mut_ptr(),
                self._wrapped,
                &mut c_size,
            )
        };
        if res == ffi::TILEDB_OK {
            Ok(c_size as u64)
        } else {
            Err(ctx.expect_last_error())
        }
    }
}

impl Drop for Attribute {
    fn drop(&mut self) {
        if self._wrapped.is_null() {
            return;
        }
        unsafe {
            ffi::tiledb_attribute_free(&mut self._wrapped);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::filter::Filter;

    #[test]
    fn attribute_alloc() {
        let ctx = Context::new().expect("Error creating context instance.");
        Attribute::new(&ctx, "foo", Datatype::UINT64)
            .expect("Error creating attribute instance.");
    }

    #[test]
    fn attribute_get_name_and_type() {
        let ctx = Context::new().expect("Error creating context instance.");
        let attr = Attribute::new(&ctx, "xkcd", Datatype::UINT32)
            .expect("Error creating attribute instance.");

        let name = attr.name(&ctx).expect("Error getting attribute name.");
        assert_eq!(&name, "xkcd");

        let dtype = attr
            .datatype(&ctx)
            .expect("Error getting attribute datatype.");
        assert_eq!(dtype, Datatype::UINT32);
    }

    #[test]
    fn attribute_set_nullable() {
        let ctx = Context::new().expect("Error creating context instance.");
        let attr = Attribute::new(&ctx, "foo", Datatype::UINT64)
            .expect("Error creating attribute instance.");

        let nullable = attr
            .get_nullable(&ctx)
            .expect("Error getting attribute nullability.");
        assert!(!nullable);

        attr.set_nullable(&ctx, true)
            .expect("Error setting attribute nullability.");

        let nullable = attr
            .get_nullable(&ctx)
            .expect("Error getting attribute nullability.");
        assert!(nullable);
    }

    #[test]
    fn attribute_get_set_filter_list() {
        let ctx = Context::new().expect("Error creating context instance.");
        let attr = Attribute::new(&ctx, "foo", Datatype::UINT8)
            .expect("Error creating attribute instance.");

        let flist1 = attr
            .get_filter_list(&ctx)
            .expect("Error getting attribute filter list.");
        let nfilters = flist1
            .get_num_filters(&ctx)
            .expect("Error getting number of filters.");
        assert_eq!(nfilters, 0);

        let f1 = Filter::new(&ctx, FilterType::NONE)
            .expect("Error creating filter 1.");
        let f2 = Filter::new(&ctx, FilterType::BIT_WIDTH_REDUCTION)
            .expect("Error creating filter 2.");
        let f3 = Filter::new(&ctx, FilterType::ZSTD)
            .expect("Error creating filter 3.");
        let flist2 =
            FilterList::new(&ctx).expect("Error creating filter list.");
        flist2
            .add_filter(&ctx, &f1)
            .expect("Error adding filter 1 to list.");
        flist2
            .add_filter(&ctx, &f2)
            .expect("Error adding filter 2 to list.");
        flist2
            .add_filter(&ctx, &f3)
            .expect("Error adding filter 3 to list.");

        attr.set_filter_list(&ctx, &flist2)
            .expect("Error setting filter list.");

        let flist3 = attr
            .get_filter_list(&ctx)
            .expect("Error getting filter list.");
        let nfilters = flist3
            .get_num_filters(&ctx)
            .expect("Error getting number of filters.");
        assert_eq!(nfilters, 3);
    }

    #[test]
    fn attribute_cell_val_size() {
        let ctx = Context::new().expect("Error creating context instance.");
        let attr = Attribute::new(&ctx, "foo", Datatype::UINT16)
            .expect("Error creating attribute instance.");

        let num = attr
            .get_cell_val_num(&ctx)
            .expect("Error getting cell val num.");
        assert_eq!(num, 1);
        let size = attr
            .get_cell_size(&ctx)
            .expect("Error getting attribute cell size.");
        assert_eq!(size, 2);

        attr.set_cell_val_num(&ctx, 3)
            .expect("Error setting cell val num.");
        let num = attr
            .get_cell_val_num(&ctx)
            .expect("Error getting cell val num.");
        assert_eq!(num, 3);
        let size = attr
            .get_cell_size(&ctx)
            .expect("Error getting attribute cell size.");
        assert_eq!(size, 6);

        attr.set_cell_val_num(&ctx, u32::MAX)
            .expect("Error setting cell val size.");
        let is_var = attr
            .is_var_sized(&ctx)
            .expect("Error getting attribute var sized-ness.");
        assert!(is_var);

        attr.set_cell_val_num(&ctx, 42)
            .expect("Error setting cell val num.");
        let num = attr
            .get_cell_val_num(&ctx)
            .expect("Error getting cell val num.");
        assert_eq!(num, 42);
        let size = attr
            .get_cell_size(&ctx)
            .expect("Error getting cell val size.");
        assert_eq!(size, 84);

        attr.set_var_sized(&ctx)
            .expect("Error setting attribute to var sized.");
        let num = attr
            .get_cell_val_num(&ctx)
            .expect("Error getting cell val num.");
        assert_eq!(num, u32::MAX);
        let size = attr
            .get_cell_size(&ctx)
            .expect("Error getting cell val size.");
        assert_eq!(size, u64::MAX);
    }
}
