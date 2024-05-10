use rand::{Rng, SeedableRng};
use rand_chacha;

#[no_mangle]
pub extern "C" fn print(n: i64) {
    println!("{}", n);
}

#[no_mangle]
pub extern "C" fn rand_int(n: i64) -> i64 {
    match std::env::var("CHIIKA_RAND_SEED") {
        Ok(seed) => {
            let seed: u64 = seed.parse().unwrap();
            println!("[chiika_runtime] Using seed: {}", seed);
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
            rng.gen_range(0..n)
        }
        _ => {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..n)
        }
    }
}
