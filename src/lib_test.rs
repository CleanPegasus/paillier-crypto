#[cfg(test)]
mod tests {
    use super::super::{generate_keypair, encrypt, decrypt, add_encrypted, subtract_encrypted};
    use num_bigint::BigInt;

    #[test]
    fn test_key_generation() {
        let (public_key, private_key) = generate_keypair(1024);
        assert!(public_key.n.bits() > 0);
        assert!(private_key.lambda.bits() > 0);
    }

    #[test]
    fn test_encryption_decryption() {
        let (public_key, private_key) = generate_keypair(1024);
        let message = BigInt::from(42);
        let ciphertext = encrypt(&public_key, &message);
        let decrypted_message = decrypt(&private_key, &ciphertext);
        assert_eq!(message, decrypted_message);
    }

    #[test]
    fn test_homomorphic_addition() {
        let (public_key, private_key) = generate_keypair(1024);
        let m1 = BigInt::from(15);
        let m2 = BigInt::from(20);
        let c1 = encrypt(&public_key, &m1);
        let c2 = encrypt(&public_key, &m2);
        let c_sum = add_encrypted(&public_key, &c1, &c2);
        let decrypted_sum = decrypt(&private_key, &c_sum);
        assert_eq!(decrypted_sum, m1 + m2);
    }

    #[test]
    fn test_homomorphic_subtraction() {
        let (public_key, private_key) = generate_keypair(1024);
        let m1 = BigInt::from(15);
        let m3 = BigInt::from(10);
        let c1 = encrypt(&public_key, &m1);
        let c3 = encrypt(&public_key, &m3);
        let c_diff = subtract_encrypted(&public_key, &c1, &c3);
        let decrypted_diff = decrypt(&private_key, &c_diff);
        assert_eq!(decrypted_diff, m1 - m3);
    }
}