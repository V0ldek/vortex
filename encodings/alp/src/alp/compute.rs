use vortex_array::array::ConstantArray;
use vortex_array::compute::unary::{scalar_at_unchecked, ScalarAtFn};
use vortex_array::compute::{
    compare, filter, slice, take, ArrayCompute, ComputeVTable, FilterFn, FilterMask,
    MaybeCompareFn, Operator, SliceFn, TakeFn, TakeOptions,
};
use vortex_array::variants::PrimitiveArrayTrait;
use vortex_array::{ArrayDType, ArrayData, ArrayLen, IntoArrayData};
use vortex_dtype::Nullability;
use vortex_error::{VortexExpect, VortexResult};
use vortex_scalar::{PValue, Scalar};

use crate::{match_each_alp_float_ptype, ALPArray, ALPEncoding, ALPFloat};

impl ArrayCompute for ALPArray {
    fn compare(&self, other: &ArrayData, operator: Operator) -> Option<VortexResult<ArrayData>> {
        MaybeCompareFn::maybe_compare(self, other, operator)
    }

    fn scalar_at(&self) -> Option<&dyn ScalarAtFn> {
        Some(self)
    }
}

impl ComputeVTable for ALPEncoding {
    fn filter_fn(&self) -> Option<&dyn FilterFn<ArrayData>> {
        Some(self)
    }

    fn slice_fn(&self) -> Option<&dyn SliceFn<ArrayData>> {
        Some(self)
    }

    fn take_fn(&self) -> Option<&dyn TakeFn<ArrayData>> {
        Some(self)
    }
}

impl ScalarAtFn for ALPArray {
    fn scalar_at(&self, index: usize) -> VortexResult<Scalar> {
        Ok(self.scalar_at_unchecked(index))
    }

    fn scalar_at_unchecked(&self, index: usize) -> Scalar {
        if let Some(patches) = self.patches() {
            if patches.with_dyn(|a| a.is_valid(index)) {
                // We need to make sure the value is actually in the patches array
                return scalar_at_unchecked(&patches, index);
            }
        }

        let encoded_val = scalar_at_unchecked(self.encoded(), index);

        match_each_alp_float_ptype!(self.ptype(), |$T| {
            let encoded_val: <$T as ALPFloat>::ALPInt = encoded_val.as_ref().try_into().unwrap();
            Scalar::primitive(<$T as ALPFloat>::decode_single(
                encoded_val,
                self.exponents(),
            ), self.dtype().nullability())
        })
    }
}

impl TakeFn<ALPArray> for ALPEncoding {
    fn take(
        &self,
        array: &ALPArray,
        indices: &ArrayData,
        options: TakeOptions,
    ) -> VortexResult<ArrayData> {
        // TODO(ngates): wrap up indices in an array that caches decompression?
        Ok(ALPArray::try_new(
            take(array.encoded(), indices, options)?,
            array.exponents(),
            array
                .patches()
                .map(|p| take(&p, indices, options))
                .transpose()?,
        )?
        .into_array())
    }
}

impl SliceFn<ALPArray> for ALPEncoding {
    fn slice(&self, array: &ALPArray, start: usize, end: usize) -> VortexResult<ArrayData> {
        Ok(ALPArray::try_new(
            slice(array.encoded(), start, end)?,
            array.exponents(),
            array.patches().map(|p| slice(&p, start, end)).transpose()?,
        )?
        .into_array())
    }
}

impl FilterFn<ALPArray> for ALPEncoding {
    fn filter(&self, array: &ALPArray, mask: FilterMask) -> VortexResult<ArrayData> {
        Ok(ALPArray::try_new(
            filter(&array.encoded(), mask.clone())?,
            array.exponents(),
            array.patches().map(|p| filter(&p, mask)).transpose()?,
        )?
        .into_array())
    }
}

impl MaybeCompareFn for ALPArray {
    fn maybe_compare(
        &self,
        array: &ArrayData,
        operator: Operator,
    ) -> Option<VortexResult<ArrayData>> {
        if let Some(const_scalar) = array.as_constant() {
            let pvalue = const_scalar
                .value()
                .as_pvalue()
                .vortex_expect("Expected primitive value");

            match pvalue {
                Some(PValue::F32(f)) => Some(alp_scalar_compare(self, f, operator)),
                Some(PValue::F64(f)) => Some(alp_scalar_compare(self, f, operator)),
                Some(_) | None => Some(Ok(ConstantArray::new(
                    Scalar::bool(false, Nullability::Nullable),
                    self.len(),
                )
                .into_array())),
            }
        } else {
            None
        }
    }
}

fn alp_scalar_compare<F: ALPFloat + Into<Scalar>>(
    alp: &ALPArray,
    value: F,
    operator: Operator,
) -> VortexResult<ArrayData>
where
    F::ALPInt: Into<Scalar>,
{
    let encoded = F::encode_single(value, alp.exponents());
    match encoded {
        Ok(encoded) => {
            let s = ConstantArray::new(encoded, alp.len());
            compare(alp.encoded(), s.as_ref(), operator)
        }
        Err(exception) => {
            if let Some(patches) = alp.patches().as_ref() {
                let s = ConstantArray::new(exception, alp.len());
                compare(patches, s.as_ref(), operator)
            } else {
                Ok(
                    ConstantArray::new(Scalar::bool(false, Nullability::Nullable), alp.len())
                        .into_array(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vortex_array::array::PrimitiveArray;
    use vortex_array::IntoArrayVariant;
    use vortex_dtype::{DType, Nullability, PType};

    use super::*;
    use crate::alp_encode;

    #[test]
    fn basic_comparison_test() {
        let array = PrimitiveArray::from(vec![1.234f32; 1025]);
        let encoded = alp_encode(&array).unwrap();
        assert!(encoded.patches().is_none());
        assert_eq!(
            encoded
                .encoded()
                .into_primitive()
                .unwrap()
                .maybe_null_slice::<i32>(),
            vec![1234; 1025]
        );

        let r = alp_scalar_compare(&encoded, 1.3_f32, Operator::Eq)
            .unwrap()
            .into_bool()
            .unwrap();

        for v in r.boolean_buffer().iter() {
            assert!(!v);
        }

        let r = alp_scalar_compare(&encoded, 1.234f32, Operator::Eq)
            .unwrap()
            .into_bool()
            .unwrap();

        for v in r.boolean_buffer().iter() {
            assert!(v);
        }
    }

    #[test]
    fn compare_with_patches() {
        let array =
            PrimitiveArray::from(vec![1.234f32, 1.5, 19.0, std::f32::consts::E, 1_000_000.9]);
        let encoded = alp_encode(&array).unwrap();
        assert!(encoded.patches().is_some());

        let r = alp_scalar_compare(&encoded, 1_000_000.9_f32, Operator::Eq)
            .unwrap()
            .into_bool()
            .unwrap();

        let buffer = r.boolean_buffer();
        assert!(buffer.value(buffer.len() - 1));
    }

    #[test]
    fn compare_to_null() {
        let array = PrimitiveArray::from(vec![1.234f32; 1025]);
        let encoded = alp_encode(&array).unwrap();

        let other = ConstantArray::new(
            Scalar::null(DType::Primitive(PType::F32, Nullability::Nullable)),
            array.len(),
        );

        let r = encoded
            .maybe_compare(other.as_ref(), Operator::Eq)
            .unwrap()
            .unwrap()
            .into_bool()
            .unwrap();

        for v in r.boolean_buffer().iter() {
            assert!(!v);
        }
    }
}
