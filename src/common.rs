use ed25519_compact::ge_scalarmult_base;
use ed25519_compact::sha512::Hash;
use rand_core::{SeedableRng, RngCore};
use rand_xorshift::XorShiftRng;
use bs58;
use libc_print::libc_println;

pub fn find_vanity_private_key(vanity_prefix: &[u8], rng_seed: u64) -> u64 {
    let vanity_prefix_len = vanity_prefix.len();

    let mut rng = XorShiftRng::seed_from_u64(rng_seed);
    let mut hasher = Hash::new();
    let mut private_key = [0u8; 32];
    let mut bs58_encoded_public_key = [0u8; 44];
    let mut num_iterations = 0;

    loop {
        // generate random input
        rng.fill_bytes(&mut private_key[0..32]);

        // sha512 hash input
        hasher.reset();
        hasher.update(&private_key[0..32]);
        let mut hashed_private_key = hasher.finalize();

        // apply ed25519 clamping
        hashed_private_key[0] &= 248;
        hashed_private_key[31] = (hashed_private_key[31] & 127) | 64;

        // ed25519 private key -> public key (first 32 bytes only)
        let public_key_bytes = ge_scalarmult_base(&hashed_private_key[0..32]).to_bytes();

        // bs58 encode public key
        bs58::encode(&public_key_bytes[0..32]).onto(&mut bs58_encoded_public_key[0..]).unwrap();

        // check if public key starts with vanity prefix
        if bs58_encoded_public_key[0..vanity_prefix_len] == *vanity_prefix {
            libc_println!("found match");
            libc_println!("Private key: {:02x?}", private_key);
            libc_println!("Public key: {:02x?}", public_key_bytes);
            libc_println!("Base58 encoded public key: {:02x?}", bs58_encoded_public_key);
            break;
        }

        num_iterations += 1;
    }

    num_iterations
}
