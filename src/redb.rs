use super::*;
use ::redb::*;

impl Key for CarettaId {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        <Self as Value>::from_bytes(data1).cmp(&<Self as Value>::from_bytes(data2))
    }
}

impl Value for CarettaId {
    type SelfType<'a> = Self;
    type AsBytes<'a> = <u64 as Value>::AsBytes<'a>;
    fn fixed_width() -> Option<usize> {
        <u64 as Value>::fixed_width()
    }
    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        Self::from_u64_lossy(<u64 as Value>::from_bytes(data))
    }
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        <u64 as Value>::as_bytes(&value.0)
    }
    fn type_name() -> TypeName {
        TypeName::new(stringify!(CarettaId))
    }
}
