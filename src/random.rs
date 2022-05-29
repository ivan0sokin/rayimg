use rand::{Rng, distributions::uniform::{SampleUniform, SampleRange}};

pub fn random_in_range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    rand::thread_rng().gen_range(range)
}
