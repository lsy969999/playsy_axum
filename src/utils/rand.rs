use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_alphanumeric_code(take: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(take)
        .map(char::from)
        .collect()
}