use crate::scalar::binary::BinaryScalar;
use crate::scalar::localtime::LocalTimeScalar;

use super::*;

impl PartialEq for dyn Scalar + '_ {
    fn eq(&self, that: &dyn Scalar) -> bool {
        equal(self, that)
    }
}

// impl PartialEq<dyn Scalar> for Arc<dyn Scalar + '_> {
//     fn eq(&self, that: &dyn Scalar) -> bool {
//         equal(&**self, that)
//     }
// }

impl PartialEq<dyn Scalar> for Box<dyn Scalar + '_> {
    fn eq(&self, that: &dyn Scalar) -> bool {
        equal(self.as_ref(), that)
    }
}

macro_rules! dyn_eq {
    ($ty:ty, $lhs:expr, $rhs:expr) => {{
        let lhs = $lhs.as_any().downcast_ref::<$ty>().unwrap();
        let rhs = $rhs.as_any().downcast_ref::<$ty>().unwrap();
        lhs == rhs
    }};
}

fn equal(lhs: &dyn Scalar, rhs: &dyn Scalar) -> bool {
    if lhs.dtype() != rhs.dtype() {
        return false;
    }

    use DType::*;
    match lhs.dtype() {
        Bool => dyn_eq!(BoolScalar, lhs, rhs),
        Int(_) => dyn_eq!(PScalar, lhs, rhs),
        UInt(_) => dyn_eq!(PScalar, lhs, rhs),
        Float(_) => dyn_eq!(PScalar, lhs, rhs),
        Struct(..) => dyn_eq!(StructScalar, lhs, rhs),
        Utf8 => dyn_eq!(Utf8Scalar, lhs, rhs),
        Binary => dyn_eq!(BinaryScalar, lhs, rhs),
        LocalTime(_) => dyn_eq!(LocalTimeScalar, lhs, rhs),
        _ => todo!("Equal not yet implemented for {:?} {:?}", lhs, rhs),
    }
}
