use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_alphanumeric_code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}