use paillier_crypto::{generate_keypair, encrypt, decrypt, add_encrypted, subtract_encrypted};
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};

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
