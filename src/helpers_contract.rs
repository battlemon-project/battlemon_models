use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use rand_for_near::{distributions::uniform::SampleUniform, rngs::StdRng, Rng, SeedableRng};

pub(crate) fn generate_id() -> String {
    let seed: [u8; 32] = near_sdk::env::random_seed().try_into().unwrap();
    let mut rng = StdRng::from_seed(seed);
    let id = rng.gen::<[u8; 9]>();
    near_sdk::base64::encode_config(&id, near_sdk::base64::URL_SAFE_NO_PAD)
}

pub fn get_random_arr_range<T, const N: usize>(begin: T, end: T) -> [T; N]
where
    T: SampleUniform + Copy + core::fmt::Debug,
{
    let seed: [u8; 32] = env::random_seed().try_into().unwrap();
    let mut rng = StdRng::from_seed(seed);
    let range: Vec<_> = (0..N).map(|_| rng.gen_range(begin, end)).collect();

    <[_; N]>::try_from(range).unwrap()
}

pub fn weights<const N: usize>() -> [u8; N] {
    get_random_arr_range(0, 100)
}

pub fn emit_log_event<'de>(model: impl Deserialize<'de> + Serialize) {
    let value = near_sdk::serde_json::to_value(model).unwrap();
    env::log_str(&format!("{}:{value}", crate::consts::EVENT_PREFIX));
}
