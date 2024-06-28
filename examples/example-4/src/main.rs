use std::{collections::HashMap, time::Instant};

use drillx::{equix::SolverMemory, Hash};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let challenge: [u8; 32] = rng.gen();
        let threads = 10;
        let handles: Vec<_> = (0..threads)
            .map(|i| {
                std::thread::spawn({
                    let mut memory = SolverMemory::new();
                    move || {
                        let timer = Instant::now();
                        let mut nonce = u64::MAX.saturating_div(threads).saturating_mul(i);
                        let mut best_nonce = nonce;
                        let mut best_difficulty = 0;
                        let mut best_hash = Hash::default();
                        loop {
                            // Create hash
                            if let Ok(hx) = drillx::hash_with_memory(
                                &mut memory,
                                &challenge,
                                &nonce.to_le_bytes(),
                            ) {
                                let difficulty = hx.difficulty();
                                if difficulty.gt(&best_difficulty) {
                                    best_nonce = nonce;
                                    best_difficulty = difficulty;
                                    best_hash = hx;
                                }
                            }

                            // Exit if time has elapsed
                            if nonce % 100 == 0 {
                                if timer.elapsed().as_secs().ge(&60) {
                                    if best_difficulty.gt(&8) {
                                        // Mine until min difficulty has been met
                                        break;
                                    }
                                } else if i == 0 {
                                    println!(
                                        "Mining... ({} sec remaining)",
                                        60u64.saturating_sub(timer.elapsed().as_secs()),
                                    );
                                }
                            }

                            // Increment nonce
                            nonce += 1;
                        }

                        // Return the best nonce
                        (best_nonce, best_difficulty, best_hash)
                    }
                })
            })
            .collect();

        // Join handles and return best nonce
        let mut best_nonce = 0;
        let mut best_difficulty = 0;
        let mut best_hash = Hash::default();
        for h in handles {
            if let Ok((nonce, difficulty, hash)) = h.join() {
                if difficulty > best_difficulty {
                    best_difficulty = difficulty;
                    best_nonce = nonce;
                    best_hash = hash;
                }
            }
        }
        println!("diff: {best_difficulty}");
    }
}

fn asd() {}

fn print(hash_counts: &HashMap<u32, u64>, timer: &Instant) {
    let max_key = *hash_counts.keys().max().unwrap();
    let mut str = format!("{} sec – ", timer.elapsed().as_secs());
    for i in 0..(max_key + 1) {
        str = format!("{} {}: {} ", str, i, hash_counts.get(&i).unwrap_or(&0)).to_string();
    }
    println!("{}", str);
}
