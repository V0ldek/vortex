use vortex_dtype::{match_each_integer_ptype, DType};
use vortex_error::{vortex_bail, VortexResult};
use vortex_scalar::Scalar;

use crate::array::null::NullArray;
use crate::array::NullEncoding;
use crate::compute::unary::ScalarAtFn;
use crate::compute::{ArrayCompute, ComputeVTable, SliceFn, TakeFn, TakeOptions};
use crate::variants::PrimitiveArrayTrait;
use crate::{ArrayData, ArrayLen, IntoArrayData, IntoArrayVariant};

impl ArrayCompute for NullArray {
    fn scalar_at(&self) -> Option<&dyn ScalarAtFn> {
        Some(self)
    }

    fn take(&self) -> Option<&dyn TakeFn> {
        Some(self)
    }
}

impl ComputeVTable for NullEncoding {
    fn slice_fn(&self) -> Option<&dyn SliceFn<ArrayData>> {
        Some(self)
    }
}

impl SliceFn<NullArray> for NullEncoding {
    fn slice(&self, _array: &NullArray, start: usize, stop: usize) -> VortexResult<ArrayData> {
        Ok(NullArray::new(stop - start).into_array())
    }
}

impl ScalarAtFn for NullArray {
    fn scalar_at(&self, index: usize) -> VortexResult<Scalar> {
        Ok(self.scalar_at_unchecked(index))
    }

    fn scalar_at_unchecked(&self, _index: usize) -> Scalar {
        Scalar::null(DType::Null)
    }
}

impl TakeFn for NullArray {
    fn take(&self, indices: &ArrayData, options: TakeOptions) -> VortexResult<ArrayData> {
        let indices = indices.clone().into_primitive()?;

        // Enforce all indices are valid
        if !options.skip_bounds_check {
            match_each_integer_ptype!(indices.ptype(), |$T| {
                for index in indices.maybe_null_slice::<$T>() {
                    if !((*index as usize) < self.len()) {
                        vortex_bail!(OutOfBounds: *index as usize, 0, self.len());
                    }
                }
            });
        }

        Ok(NullArray::new(indices.len()).into_array())
    }
}

#[cfg(test)]
mod test {
    use vortex_dtype::DType;

    use crate::array::null::NullArray;
    use crate::array::ConstantArray;
    use crate::compute::unary::scalar_at;
    use crate::compute::{slice, TakeFn, TakeOptions};
    use crate::validity::{ArrayValidity, LogicalValidity};
    use crate::{ArrayLen, IntoArrayData};

    #[test]
    fn test_slice_nulls() {
        let nulls = NullArray::new(10);

        // Turns out, the slice function has a short-cut for constant arrays.
        // Sooo... we get back a constant!
        let sliced = ConstantArray::try_from(slice(nulls.into_array(), 0, 4).unwrap()).unwrap();

        assert_eq!(sliced.len(), 4);
        assert!(matches!(
            sliced.logical_validity(),
            LogicalValidity::AllInvalid(4)
        ));
    }

    #[test]
    fn test_take_nulls() {
        let nulls = NullArray::new(10);
        let taken = NullArray::try_from(
            nulls
                .take(&vec![0u64, 2, 4, 6, 8].into_array(), TakeOptions::default())
                .unwrap(),
        )
        .unwrap();

        assert_eq!(taken.len(), 5);
        assert!(matches!(
            taken.logical_validity(),
            LogicalValidity::AllInvalid(5)
        ));
    }

    #[test]
    fn test_scalar_at_nulls() {
        let nulls = NullArray::new(10);

        let scalar = scalar_at(&nulls, 0).unwrap();
        assert!(scalar.is_null());
        assert_eq!(scalar.dtype().clone(), DType::Null);
    }
}
