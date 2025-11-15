use super::*;
use ::rand::{distr::{Distribution, StandardUniform}, Rng};

impl Distribution<CarettaId> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CarettaId {
        <CarettaId>::from_u64_lossy(rng.random())
    }
}
impl CarettaId {
    /// Generate a new random [`CarettaId`].
    ///
    /// This method generate a random ID.
    /// The generated ID is guaranteed to not be the [`NIL`](Self::NIL) value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caretta_id::*;
    /// let id = CarettaId::random();
    /// assert_ne!(id, CarettaId::NIL);
    /// ```
    pub fn random() -> Self {
        <CarettaId>::from_u64_lossy(::rand::random())
    }
}