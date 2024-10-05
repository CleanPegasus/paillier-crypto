use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use num_integer::Integer;

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
fn generate_prime(bit_length: usize) -> BigInt {
    let mut rng = rand::thread_rng();
    loop {
        let n: BigInt = rng.gen_bigint(bit_length as u64);
        if n.bits() as usize != bit_length {
            continue;
        }
        if is_probable_prime(&n, 20) {
            return n;
        }
    }
}

// Miller-Rabin Primality test
fn is_probable_prime(n: &BigInt, k: usize) -> bool {
    if n <= &BigInt::one() {
        return false;
    }
    if n == &BigInt::from(2) || n == &BigInt::from(3) {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let mut d: BigInt = n - 1;
    let mut s = 0;
    while d.is_even() {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a: BigInt = rng.gen_bigint_range(&BigInt::from(2), &(n - 2));
        let mut x = a.modpow(&d, n);
        if x == BigInt::one() || x == n - 1 {
            continue;
        }
        let mut is_composite = true;
        for _ in 0..s - 1 {
            x = x.modpow(&BigInt::from(2), n);
            if x == n - 1 {
                is_composite = false;
                break;
            }
        }
        if is_composite {
            return false;
        }
    }
    true
}

fn generate_keypair(bit_length: usize) -> (PaillierPublicKey, PaillierPrivateKey) {

    let p = generate_prime(bit_length / 2);
    let q = generate_prime(bit_length / 2);
    
    let n = &p * &q;
    let n_squared = &n * &n;
    
    let lambda = lcm(&(p - 1), &(q - 1));
    let g: BigInt = &n + 1;
    
    let mu = mod_inverse(&l(&g.modpow(&lambda, &n_squared), &n), &n);
    
    let public_key = PaillierPublicKey { n: n.clone(), g: g.clone() };
    let private_key = PaillierPrivateKey { lambda, mu, public_key: public_key.clone() };
    
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
    (l(&c.modpow(&private_key.lambda, &n_squared), &private_key.public_key.n) * &private_key.mu) % &private_key.public_key.n
}

// Homomorphic addition of encrypted values
fn add_encrypted(public_key: &PaillierPublicKey, c1: &BigInt, c2: &BigInt) -> BigInt {
    let n_squared = &public_key.n * &public_key.n;
    (c1 * c2) % &n_squared
}

// Helper functions
fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    (a * b) / a.gcd(b)
}

fn l(x: &BigInt, n: &BigInt) -> BigInt {
    (x - 1) / n
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let mut mn = (m.clone(), a.clone());
    let mut xy = (BigInt::zero(), BigInt::one());

    while mn.1 != BigInt::zero() {
        xy = (xy.1.clone(), xy.0 - (&mn.0 / &mn.1) * &xy.1);
        mn = (mn.1.clone(), &mn.0 % &mn.1);
    }

    while xy.0 < BigInt::zero() {
        xy.0 += m;
    }
    xy.0
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
}