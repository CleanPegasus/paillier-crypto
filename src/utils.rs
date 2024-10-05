use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use num_integer::Integer;

// Helper functions
pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
  (a * b) / a.gcd(b)
}

pub fn l(x: &BigInt, n: &BigInt) -> BigInt {
  (x - 1) / n
}

pub fn generate_prime(bit_length: usize) -> BigInt {
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
pub fn is_probable_prime(n: &BigInt, k: usize) -> bool {
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



pub fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
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


pub fn mod_pow(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
  let mut result = BigInt::one();
  let mut base = base.clone();
  let mut exp = exponent.clone();
  
  while exp > BigInt::zero() {
      if &exp % 2 == BigInt::one() {
          result = (result * &base) % modulus;
      }
      base = (&base * &base) % modulus;
      exp /= 2;
  }
  result
}