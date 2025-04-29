use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn main() {

    // =====================
    // 1. Parse command-line arguments
    // Expect exactly two arguments: prng_seed and num_iterations
    // =====================

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./fuzzer <prng_seed> <num_iterations>");
        std::process::exit(1);
    }

    let prng_seed: u32 = args[1].parse().expect("Invalid prng_seed");
    let num_iterations: usize = args[2].parse().expect("Invalid num_iterations");

    // =====================
    // Read the initial input seed file
    // The _seed_ file must exist in the working directory
    // This is the base input that will be mutated
    // =====================

    let mut seed_file = File::open("_seed_").expect("Failed to open _seed_ file");
    let mut input_seed = Vec::new();
    seed_file.read_to_end(&mut input_seed).expect("Failed to read _seed_ file");

    // =====================
    // Use the provided prng_seed for deterministic behavior
    // =====================

    let mut rng = StdRng::seed_from_u64(prng_seed);

    // =====================
    // Begin mutation
    // - Passes determined by num_iterations 
    // - Each pass, 13% chance mutate bytes
    // - Every 500 iterations append 10 random bytes
    // =====================
    
    let mut mutated_data = input_seed.clone();

    for i in 1..=num_iterations {
        for byte in mutated_data.iter_mut() {
            if rng.gen_range(0..100) < 13 {
                *byte = rng.gen_range(0..=255);
            }
        }

        if i % 500 == 0 {
            for _ in 0..10 {
                mutated_data.push(rng.gen_range(0..=255));
            }
        }
    }

    // =====================
    // Outputs the final mutated data to stdout
    // Stdout should be redirected to a file when running to capture the payload used
    // =====================

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&mutated_data).expect("Failed to write output");
}