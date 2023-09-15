use crate::{Dummy, Fake, Faker};
use rand::Rng;
use std::collections::BTreeSet;

impl<T> Dummy<Faker> for BTreeSet<T>
where
    T: Dummy<Faker> + Ord,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = super::get_len(config, rng);
        let mut m = BTreeSet::new();
        for _ in 0..len {
            m.insert(config.fake_with_rng(rng));
        }
        m
    }
}
