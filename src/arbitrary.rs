use super::*;
use ::arbitrary::{Arbitrary, Result, Unstructured};
impl<'a> Arbitrary<'a> for CarettaId {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self::from_u64_lossy(u64::arbitrary(u)?))
    }
}
