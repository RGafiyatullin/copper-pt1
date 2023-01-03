pub trait Value: Copy + Eq + std::fmt::Debug {
    fn zero() -> Self;
    fn wrapping_add_assign(&mut self, addend: &Self);

    fn split<const N: usize, R: rand::Rng>(&self, rng: &mut R) -> [Self; N];

    #[cfg(test)]
    fn pick_random<R: rand::Rng>(rng: &mut R) -> Self;
}

macro_rules! impl_value_for {
    ($t: ty) => {
        impl Value for $t {
            fn zero() -> Self {
                0
            }
            fn wrapping_add_assign(&mut self, other: &Self) {
                *self = self.wrapping_add(*other);
            }

            fn split<const N: usize, R: rand::Rng>(&self, rng: &mut R) -> [Self; N] {
                let mut shares: [Self; N] = std::array::from_fn(|_| rng.gen::<Self>());

                let mut sum: Self = Default::default();
                for attempt in shares.iter().copied() {
                    sum.wrapping_add_assign(&attempt);
                }

                let miss = self.wrapping_sub(sum);
                let n_as_v = N as Self;
                let miss_quot = miss / n_as_v;
                let miss_rem = miss % n_as_v;

                for s in shares.iter_mut() {
                    s.wrapping_add_assign(&miss_quot);
                }
                shares[rng.gen::<usize>() % shares.len()].wrapping_add_assign(&miss_rem);

                shares
            }

            #[cfg(test)]
            fn pick_random<R: rand::Rng>(rng: &mut R) -> Self {
                rng.gen()
            }
        }
    };
}

impl_value_for!(i8);
impl_value_for!(i16);
impl_value_for!(i32);
impl_value_for!(i64);
impl_value_for!(i128);

impl_value_for!(u8);
impl_value_for!(u16);
impl_value_for!(u32);
impl_value_for!(u64);
impl_value_for!(u128);
