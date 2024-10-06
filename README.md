# Paillier Cryptosystem Library

This Rust library implements the Paillier cryptosystem, a probabilistic asymmetric algorithm for public key cryptography. The library provides functionality for key generation, encryption, decryption, and homomorphic operations.

### Features
- **Key Generation**: Generate public and private key pairs.
- **Encryption**: Encrypt messages using the public key.
- **Decryption**: Decrypt ciphertexts using the private key.
- **Homomorphic Operations**: Perform addition and subtraction on encrypted values without decrypting them.

### Installation
Add the following to your Cargo.toml:
```bash
cargo add paillier-crypto
```

### Usage
- Key Generation: Generate a public and private key pair:

```rust
use paillier_crypto::{generate_keypair, PaillierPublicKey, PaillierPrivateKey};
let (public_key, private_key) = generate_keypair(512);
```

- Encryption: Encrypt a message using the public key:

```rust
use num_bigint::ToBigInt;

let message = 42.to_bigint().unwrap();
let ciphertext = encrypt(&public_key, &message);
```

- Decryption: Decrypt a ciphertext using the private key:

```rust
let decrypted_message = decrypt(&private_key, &ciphertext);
```

- Homomorphic Operations: Perform homomorphic addition and subtraction on encrypted values:

```rust
let message1 = 42.to_bigint().unwrap();
let message2 = 15.to_bigint().unwrap();

let ciphertext1 = encrypt(&public_key, &message1);
let ciphertext2 = encrypt(&public_key, &message2);

let sum_ciphertext = add_encrypted(&public_key, &ciphertext1, &ciphertext2);
let diff_ciphertext = subtract_encrypted(&public_key, &ciphertext1, &ciphertext2);

let sum = decrypt(&private_key, &sum_ciphertext);
let diff = decrypt(&private_key, &diff_ciphertext);
```


### Example
Here is a complete example demonstrating key generation, encryption, decryption, and homomorphic operations:

```rust
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
```

### ToD0
- [ ] Homomorphic multiplication
- [ ] Multiplicative inverse
- [ ] Homomorphic division

### License
This project is licensed under the MIT License. See the LICENSE file for details.

### Contributing
Contributions are welcome! Please open an issue or submit a pull request.

### References
- [Paillier Cryptosystem](https://en.wikipedia.org/wiki/Paillier_cryptosystem)