use rand::{thread_rng, Rng};

pub(crate) fn random_zero_one() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}