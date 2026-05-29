/// Xorshift32 PRNG — custom pseudo-random number generator (no std::random)
/// Algorithm: x ^= x << 13; x ^= x >> 17; x ^= x ^ 5
/// Period: 2^32 - 1
#[derive(Debug, Clone)]
pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    /// Create a new Xorshift32 generator with the given seed.
    /// Seed must be non-zero (0 is a degenerate state).
    pub fn new(seed: u32) -> Self {
        Xorshift32 {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// Generate the next pseudo-random u32 value
    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x.wrapping_shl(13);
        x ^= x.wrapping_shr(17);
        x ^= x.wrapping_shl(5);
        self.state = x;
        x
    }

    /// Generate a pseudo-random byte (low 8 bits of next u32)
    pub fn next_byte(&mut self) -> u8 {
        (self.next() & 0xFF) as u8
    }
}

/// Generate a gamma (byte slice) of the requested length using the given seed.
pub fn generate_gamma(length: usize, seed: u32) -> Vec<u8> {
    let mut rng = Xorshift32::new(seed);
    (0..length).map(|_| rng.next_byte()).collect()
}

/// Encrypt plaintext using gamming (XOR with PRNG-generated gamma).
/// Returns the ciphertext as a Vec<u8>.
pub fn gamming_encrypt(plaintext: &str, seed: u32) -> Vec<u8> {
    let bytes = plaintext.as_bytes();
    let gamma = generate_gamma(bytes.len(), seed);
    bytes.iter().zip(gamma).map(|(&b, g)| b ^ g).collect()
}

/// Decrypt ciphertext using gamming (XOR with PRNG-generated gamma).
/// Returns the decrypted string (assumes valid UTF-8 result).
pub fn gamming_decrypt(ciphertext: &[u8], seed: u32) -> String {
    let gamma = generate_gamma(ciphertext.len(), seed);
    let bytes: Vec<u8> = ciphertext.iter().zip(gamma).map(|(&b, g)| b ^ g).collect();
    String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift_non_zero() {
        let mut rng = Xorshift32::new(42);
        for _ in 0..100 {
            assert_ne!(rng.next(), 0);
        }
    }

    #[test]
    fn test_zero_seed_reset() {
        let mut rng = Xorshift32::new(0);
        // Should not be stuck at 0
        assert_ne!(rng.next(), 0);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "Hello, Мир! 你好";
        let seed = 12345;
        let encrypted = gamming_encrypt(original, seed);
        let decrypted = gamming_decrypt(&encrypted, seed);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_wrong_seed_fails() {
        let original = "Secret";
        let encrypted = gamming_encrypt(original, 42);
        let decrypted = gamming_decrypt(&encrypted, 99);
        assert_ne!(original, decrypted);
    }

    #[test]
    fn test_gamma_deterministic() {
        let g1 = generate_gamma(20, 777);
        let g2 = generate_gamma(20, 777);
        assert_eq!(g1, g2);
    }
}