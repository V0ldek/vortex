use vortex_array::compute::unary::ScalarAtFn;
use vortex_array::compute::{ArrayCompute, FilterFn, SearchSortedFn, SliceFn, TakeFn};

use crate::BitPackedArray;

mod filter;
mod scalar_at;
mod search_sorted;
mod slice;
mod take;

impl ArrayCompute for BitPackedArray {
    fn filter(&self) -> Option<&dyn FilterFn> {
        Some(self)
    }

    fn scalar_at(&self) -> Option<&dyn ScalarAtFn> {
        Some(self)
    }

    fn search_sorted(&self) -> Option<&dyn SearchSortedFn> {
        Some(self)
    }

    fn slice(&self) -> Option<&dyn SliceFn> {
        Some(self)
    }

    fn take(&self) -> Option<&dyn TakeFn> {
        Some(self)
    }
}
