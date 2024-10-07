use rand::Rng;

// Generates a private key which is a random number in the range [2, p-1]
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

// Modular exponentiation function to compute (base^exp) % modular efficiently
fn modular_exponentiation(base: u128, exp: u64, modular: u64) -> u64 {
    let mut e = exp;
    let mut b = base;
    let mut result = 1;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modular as u128;
        }
        b = (b * b) % modular as u128;
        e /= 2;
    }

    result as u64
}

// Computes the public key: (g^a) % p
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g as u128, a, p)
}

// Computes the secret key: (b_pub^a) % p
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub as u128, a, p)
}

fn main() {
    // Prime number (p) and primitive root modulo (g)
    let p: u64 = 23;  // Small prime for demonstration
    let g: u64 = 5;   // Primitive root modulo p

    // Generate private keys for Alice and Bob
    let alice_private = private_key(p);
    let bob_private = private_key(p);

    // Generate public keys for Alice and Bob
    let alice_public = public_key(p, g, alice_private);
    let bob_public = public_key(p, g, bob_private);

    // Exchange public keys and compute the shared secret
    let alice_shared_secret = secret(p, bob_public, alice_private);
    let bob_shared_secret = secret(p, alice_public, bob_private);

    // Output the results
    println!("Prime (p): {}", p);
    println!("Generator (g): {}", g);
    println!("Alice's Private Key: {}", alice_private);
    println!("Bob's Private Key: {}", bob_private);
    println!("Alice's Public Key: {}", alice_public);
    println!("Bob's Public Key: {}", bob_public);
    println!("Alice's Shared Secret: {}", alice_shared_secret);
    println!("Bob's Shared Secret: {}", bob_shared_secret);

    // The shared secret should be the same for both Alice and Bob
    assert_eq!(alice_shared_secret, bob_shared_secret);
    println!("Key exchange successful! Shared secret is the same.");
}
