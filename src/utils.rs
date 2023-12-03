use rand::{distributions::Alphanumeric, Rng};

pub fn generate_string(length: Option<usize>) -> String {
    let length = length.unwrap_or_else(|| 7);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
