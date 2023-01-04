use crate::value::Value;

#[derive(Debug)]
pub(super) enum State<const N: usize, V> {
    Round1Sending([Option<V>; N]),
    Round2Receiving([Option<V>; N]),
    Round2Sending([Option<V>; N]),
    Round3Receiving([Option<V>; N]),
    Round4Sending([Option<V>; N]),
    Round4Receiving([Option<V>; N]),
}

impl<const N: usize, V> State<N, V>
where
    V: Value,
{
    pub(super) fn init(secret: V) -> Self
    where
        V: Value,
    {
        Self::Round1Sending(secret.split(&mut rand::rngs::OsRng).map(Some))
    }
}

#[test]
fn test_state_init() {
    const TIMES: usize = 100;

    test_state_init_impl::<1>(TIMES);
    test_state_init_impl::<2>(TIMES);
    test_state_init_impl::<3>(TIMES);
    test_state_init_impl::<4>(TIMES);
    test_state_init_impl::<5>(TIMES);

    fn test_state_init_impl<const N: usize>(times: usize) {
        let rng = &mut rand::rngs::OsRng;

        for _ in 0..times {
            let target = i128::pick_random(rng);

            let State::Round1Sending(shares) = State::<N, _>::init(target) else { panic!("invalid state") };

            assert_eq!(
                shares
                    .into_iter()
                    .flatten()
                    .reduce(|left, right| left.wrapping_add(right)),
                Some(target)
            );
        }
    }
}
