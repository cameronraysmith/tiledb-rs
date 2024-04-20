use super::*;

use crate::error::Error;
use crate::query::buffer::QueryBuffersMut;

/// Encapsulates data for writing intermediate query results for a data field.
pub(crate) struct RawReadHandle<'data, C> {
    /// Name of the field which this handle receives data from
    pub field: String,

    /// As input to the C API, the size of the data buffer.
    /// As output from the C API, the size in bytes of an intermediate result.
    pub data_size: Pin<Box<u64>>,

    /// As input to the C API, the size of the cell offsets buffer.
    /// As output from the C API, the size in bytes of intermediate offset results.
    pub offsets_size: Option<Pin<Box<u64>>>,

    /// Buffers for writing data and cell offsets.
    /// These are re-registered with the query at each step.
    /// The application which owns the query may own these buffers,
    /// or defer their management to the reader.
    // In the case of the former, the application can do whatever it wants with the
    // buffers between steps of a query.
    // RefCell is used so that the query can write to the buffers when it is executing
    // but the application can do whatever with the buffers between steps.
    pub location: &'data RefCell<QueryBuffersMut<'data, C>>,
}

impl<'data, C> RawReadHandle<'data, C> {
    pub fn new<S>(
        field: S,
        location: &'data RefCell<QueryBuffersMut<'data, C>>,
    ) -> Self
    where
        S: AsRef<str>,
    {
        let (data, cell_offsets) = {
            let mut scratch: RefMut<QueryBuffersMut<'data, C>> =
                location.borrow_mut();

            let data = scratch.data.as_mut() as *mut [C];
            let data = unsafe { &mut *data as &mut [C] };

            let cell_offsets = scratch.cell_offsets.as_mut().map(|c| {
                let c = c.as_mut() as *mut [u64];
                unsafe { &mut *c as &mut [u64] }
            });

            (data, cell_offsets)
        };

        let data_size = Box::pin(std::mem::size_of_val(&*data) as u64);

        let offsets_size = cell_offsets.as_ref().map(|off| {
            let sz = std::mem::size_of_val::<[u64]>(*off);
            Box::pin(sz as u64)
        });

        RawReadHandle {
            field: field.as_ref().to_string(),
            data_size,
            offsets_size,
            location,
        }
    }

    pub(crate) fn attach_query(
        &mut self,
        context: &Context,
        c_query: *mut ffi::tiledb_query_t,
    ) -> TileDBResult<()> {
        let c_context = context.capi();
        let c_name = cstring!(&*self.field);

        let mut location = self.location.borrow_mut();

        *self.data_size.as_mut() =
            std::mem::size_of_val::<[C]>(&location.data) as u64;

        context.capi_return({
            let data = &mut location.data;
            let c_bufptr = data.as_mut().as_ptr() as *mut std::ffi::c_void;
            let c_sizeptr = self.data_size.as_mut().get_mut() as *mut u64;

            unsafe {
                ffi::tiledb_query_set_data_buffer(
                    c_context,
                    c_query,
                    c_name.as_ptr(),
                    c_bufptr,
                    c_sizeptr,
                )
            }
        })?;

        let cell_offsets = &mut location.cell_offsets;

        if let Some(ref mut offsets_size) = self.offsets_size.as_mut() {
            let cell_offsets = cell_offsets.as_mut().unwrap();

            *offsets_size.as_mut() =
                std::mem::size_of_val::<[u64]>(cell_offsets) as u64;

            let c_offptr = cell_offsets.as_mut_ptr();
            let c_sizeptr = offsets_size.as_mut().get_mut() as *mut u64;

            context.capi_return(unsafe {
                ffi::tiledb_query_set_offsets_buffer(
                    c_context,
                    c_query,
                    c_name.as_ptr(),
                    c_offptr,
                    c_sizeptr,
                )
            })?;
        }

        Ok(())
    }

    /// Returns the number of records and bytes produced by the last read,
    /// or the capacity of the destination buffers if no read has occurred.
    pub fn last_read_size(&self) -> (usize, usize) {
        let nvalues = match self.offsets_size.as_ref() {
            Some(offsets_size) => {
                **offsets_size as usize / std::mem::size_of::<u64>()
            }
            None => *self.data_size as usize / std::mem::size_of::<C>(),
        };
        let nbytes = *self.data_size as usize;

        (nvalues, nbytes)
    }
}

/// Reads query results into a raw buffer.
/// This is the most flexible way to read data but also the most cumbersome.
/// Recommended usage is to run the query one step at a time, and borrow
/// the buffers between each step to process intermediate results.
#[derive(ContextBound, Query)]
pub struct RawReadQuery<'data, C, Q> {
    pub(crate) raw_read_output: RawReadHandle<'data, C>,
    #[base(ContextBound, Query)]
    pub(crate) base: Q,
}

impl<'ctx, 'data, C, Q> ReadQuery<'ctx> for RawReadQuery<'data, C, Q>
where
    Q: ReadQuery<'ctx>,
{
    type Intermediate = (usize, usize, Q::Intermediate);
    type Final = (usize, usize, Q::Final);

    fn step(
        &mut self,
    ) -> TileDBResult<ReadStepOutput<Self::Intermediate, Self::Final>> {
        /* update the internal buffers */
        self.raw_read_output
            .attach_query(self.base().context(), **self.base().cquery())?;

        /* then execute */
        let base_result = {
            let _ = self.raw_read_output.location.borrow_mut();
            self.base.step()?
        };

        let (nvalues, nbytes) = self.raw_read_output.last_read_size();

        Ok(match base_result {
            ReadStepOutput::NotEnoughSpace => {
                /* TODO: check that records/bytes are zero and produce an internal error if not */
                ReadStepOutput::NotEnoughSpace
            }
            ReadStepOutput::Intermediate(base_result) => {
                if nvalues == 0 && nbytes == 0 {
                    ReadStepOutput::NotEnoughSpace
                } else if nvalues == 0 {
                    return Err(Error::Internal(format!(
                        "Invalid read: returned {} offsets but {} bytes",
                        nvalues, nbytes
                    )));
                } else {
                    ReadStepOutput::Intermediate((nvalues, nbytes, base_result))
                }
            }
            ReadStepOutput::Final(base_result) => {
                ReadStepOutput::Final((nvalues, nbytes, base_result))
            }
        })
    }
}

#[derive(ContextBound)]
pub struct RawReadBuilder<'data, C, B> {
    pub(crate) raw_read_output: RawReadHandle<'data, C>,
    #[base(ContextBound)]
    pub(crate) base: B,
}

impl<'ctx, 'data, C, B> QueryBuilder<'ctx> for RawReadBuilder<'data, C, B>
where
    B: QueryBuilder<'ctx>,
{
    type Query = RawReadQuery<'data, C, B::Query>;

    fn base(&self) -> &BuilderBase<'ctx> {
        self.base.base()
    }

    fn build(self) -> Self::Query {
        RawReadQuery {
            raw_read_output: self.raw_read_output,
            base: self.base.build(),
        }
    }
}

impl<'ctx, 'data, C, B> ReadQueryBuilder<'ctx> for RawReadBuilder<'data, C, B> where
    B: ReadQueryBuilder<'ctx>
{
}