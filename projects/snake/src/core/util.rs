use rand::Rng;

pub(super) fn generate_random_position() -> (u32, u32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=500), rng.gen_range(1..=500))
}
