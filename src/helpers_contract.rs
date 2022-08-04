use rand::{Rng, SeedableRng};

pub(crate) fn generate_id() -> String {
    let seed: [u8; 32] = near_sdk::env::random_seed().try_into().unwrap();
    let mut rng = rand::rngs::StdRng::from_seed(seed);
    let id = rng.gen::<[u8; 9]>();
    near_sdk::base64::encode_config(&id, near_sdk::base64::URL_SAFE_NO_PAD)
}
