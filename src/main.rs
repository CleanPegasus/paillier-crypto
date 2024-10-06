mod utils;
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};

use utils::{generate_prime, l, lcm, mod_inverse, mod_pow};

#[derive(Debug, Clone)]
struct PaillierPublicKey {
    n: BigInt,
    g: BigInt,
}

#[derive(Debug, Clone)]
struct PaillierPrivateKey {
    lambda: BigInt,
    mu: BigInt,
    public_key: PaillierPublicKey,
}

fn generate_keypair(bit_length: usize) -> (PaillierPublicKey, PaillierPrivateKey) {
    let p = generate_prime(bit_length / 2);
    let q = generate_prime(bit_length / 2);

    let n = &p * &q;
    let n_squared = &n * &n;

    let lambda = lcm(&(p - 1), &(q - 1));
    let g: BigInt = &n + 1;

    let mu = mod_inverse(&l(&g.modpow(&lambda, &n_squared), &n), &n);

    let public_key = PaillierPublicKey {
        n: n.clone(),
        g: g.clone(),
    };
    let private_key = PaillierPrivateKey {
        lambda,
        mu,
        public_key: public_key.clone(),
    };

    (public_key, private_key)
}

fn encrypt(public_key: &PaillierPublicKey, m: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    let r: BigInt = rng.gen_bigint_range(&BigInt::one(), &public_key.n);
    let n_squared = &public_key.n * &public_key.n;

    (public_key.g.modpow(m, &n_squared) * r.modpow(&public_key.n, &n_squared)) % &n_squared
}

fn decrypt(private_key: &PaillierPrivateKey, c: &BigInt) -> BigInt {
    let n_squared = &private_key.public_key.n * &private_key.public_key.n;
    (l(
        &c.modpow(&private_key.lambda, &n_squared),
        &private_key.public_key.n,
    ) * &private_key.mu)
        % &private_key.public_key.n
}

// Homomorphic addition of encrypted values
fn add_encrypted(public_key: &PaillierPublicKey, c1: &BigInt, c2: &BigInt) -> BigInt {
    let n_squared = &public_key.n * &public_key.n;
    (c1 * c2) % &n_squared
}

fn additive_inverse(public_key: &PaillierPublicKey, c: &BigInt) -> BigInt {
    let n_squared = &public_key.n * &public_key.n;
    mod_pow(c, &(&public_key.n - BigInt::one()), &n_squared)
}

// Homomorphic subtraction of encrypted values
fn subtract_encrypted(public_key: &PaillierPublicKey, c1: &BigInt, c2: &BigInt) -> BigInt {
    let inverse_c2 = additive_inverse(public_key, c2);
    add_encrypted(public_key, c1, &inverse_c2)
}

fn main() {
    let (public_key, private_key) = generate_keypair(1024);

    let m1 = BigInt::from(15);
    let m2 = BigInt::from(20);

    let c1 = encrypt(&public_key, &m1);
    let c2 = encrypt(&public_key, &m2);

    println!("m1: {}", m1);
    println!("m2: {}", m2);

    let decrypted_m1 = decrypt(&private_key, &c1);
    let decrypted_m2 = decrypt(&private_key, &c2);

    println!("Decrypted m1: {}", decrypted_m1);
    println!("Decrypted m2: {}", decrypted_m2);

    let c_sum = add_encrypted(&public_key, &c1, &c2);
    let decrypted_sum = decrypt(&private_key, &c_sum);

    println!("Decrypted sum: {}", decrypted_sum);
    println!("Actual sum: {}", &m1 + &m2);

    let m3 = BigInt::from(10);
    let c3 = encrypt(&public_key, &m3);

    let c_diff = subtract_encrypted(&public_key, &c1, &c3);
    let decrypted_diff = decrypt(&private_key, &c_diff);

    println!("m1: {}", m1);
    println!("m3: {}", m3);
    println!("Decrypted difference (m1 - m3): {}", decrypted_diff);
    println!("Actual difference: {}", &m1 - &m3);
}
