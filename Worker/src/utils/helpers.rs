use rand::Rng;

pub fn generate_token() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789\
                           !@#$%^&*()_+-=<>?";
    let mut rng = rand::thread_rng();

    let token: String = (0..150)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    token
}
pub fn random_id() -> u128 {
    rand::thread_rng().gen_range(100_000_000_000_000_000_000_000_000_000_000_000_000..=340282366920938463463374607431768211455)
}


