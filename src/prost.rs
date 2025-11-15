use crate::{CarettaId, CarettaIdProto, Error};

impl From<CarettaId> for CarettaIdProto {
    fn from(value: CarettaId) -> Self {
        Self { value: value.into() }
    }
}

impl TryFrom<CarettaIdProto> for CarettaId {
    type Error = Error;
    fn try_from(value: CarettaIdProto) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

impl CarettaId {

    /// "Converts a [`crate::proto::CarettaId`] to [`CarettaId`] by truncating bits that exceed the valid range.")]
    ///
    /// This is a lossy conversion that masks the input value to fit within the ID's bit limit.
    /// If you need to detect out-of-range values, use [`TryFrom`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// // Values within range are preserved
    /// let valid_proto = CarettaIdProto{value: 123};
    /// let id = CarettaId::from_proto_lossy(valid_proto);
    /// assert_eq!(<CarettaIdProto>::from(id), valid_proto);
    ///
    /// // values exceeding 35 bits are truncated (MSB(s) dropped
    /// 
    /// let oversized_proto = CarettaIdProto{value: valid_proto.value + CarettaId::CAPACITY};
    /// let overflowed_id = CarettaId::from_proto_lossy(oversized_proto);
    /// assert_ne!(CarettaIdProto::from(overflowed_id), oversized_proto);
    /// // Only lower 35 bits retained
    /// assert_eq!(CarettaIdProto::from(overflowed_id), valid_proto);
    /// ```
    pub fn from_proto_lossy(value: CarettaIdProto) -> Self {
        Self::from_u64_lossy(value.value)
    }
}