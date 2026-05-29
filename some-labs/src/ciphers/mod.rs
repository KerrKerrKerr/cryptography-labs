use std::collections::{HashMap, HashSet};

pub fn atbash_cipher(input: &str) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    fn map_from_alphabet(c: char, alphabet: &[char]) -> Option<char> {
        alphabet
            .iter()
            .position(|&letter| letter == c)
            .map(|idx| alphabet[alphabet.len() - 1 - idx])
    }

    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let offset = c as u32 - 'a' as u32;
                char::from_u32('z' as u32 - offset).unwrap_or(c)
            } else if c.is_ascii_uppercase() {
                let offset = c as u32 - 'A' as u32;
                char::from_u32('Z' as u32 - offset).unwrap_or(c)
            } else if let Some(mapped) = map_from_alphabet(c, &RU_LOWER) {
                mapped
            } else if let Some(mapped) = map_from_alphabet(c, &RU_UPPER) {
                mapped
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_cipher(input: &str, shift: i32) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    fn shift_ascii(c: char, base: char, shift: i32) -> char {
        let idx = c as i32 - base as i32;
        let normalized = (idx + shift).rem_euclid(26);
        char::from_u32((base as u32) + normalized as u32).unwrap_or(c)
    }

    fn shift_in_alphabet(c: char, alphabet: &[char], shift: i32) -> Option<char> {
        let len = alphabet.len() as i32;
        alphabet
            .iter()
            .position(|&letter| letter == c)
            .map(|idx| alphabet[(idx as i32 + shift).rem_euclid(len) as usize])
    }

    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                shift_ascii(c, 'a', shift)
            } else if c.is_ascii_uppercase() {
                shift_ascii(c, 'A', shift)
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_LOWER, shift) {
                mapped
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_UPPER, shift) {
                mapped
            } else {
                c
            }
        })
        .collect()
}

pub fn parse_rishelau_mask(mask: &str) -> Result<Vec<Vec<usize>>, String> {
    let trimmed = mask.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let groups: Vec<&str> = trimmed
        .split('|')
        .map(str::trim)
        .filter(|group| !group.is_empty())
        .collect();

    if groups.is_empty() {
        return Err("Mask must contain at least one group.".to_string());
    }

    let mut parsed_groups = Vec::with_capacity(groups.len());

    for (group_idx, group) in groups.iter().enumerate() {
        let numbers: Vec<usize> = group
            .split(|ch: char| !ch.is_ascii_digit())
            .filter(|part| !part.is_empty())
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("Group {} has invalid number.", group_idx + 1))?;

        if numbers.is_empty() {
            return Err(format!("Group {} is empty.", group_idx + 1));
        }

        if numbers.len() < 2 {
            return Err(format!("Group {} must have at least 2 numbers.", group_idx + 1));
        }

        if numbers.iter().any(|&num| num == 0) {
            return Err(format!("Group {} contains 0, but positions start at 1.", group_idx + 1));
        }

        let mut seen = HashSet::with_capacity(numbers.len());
        for &num in &numbers {
            if !seen.insert(num) {
                return Err(format!("Group {} has duplicate value {}.", group_idx + 1, num));
            }
        }

        let expected_len = numbers.len();
        if numbers.iter().any(|&num| num > expected_len) {
            return Err(format!(
                "Group {} must be a permutation of 1..{}.",
                group_idx + 1,
                expected_len
            ));
        }

        for expected in 1..=expected_len {
            if !seen.contains(&expected) {
                return Err(format!(
                    "Group {} is missing value {} (must be permutation of 1..{}).",
                    group_idx + 1,
                    expected,
                    expected_len
                ));
            }
        }

        parsed_groups.push(numbers);
    }

    Ok(parsed_groups)
}

pub fn rishelau_cipher(text: &str, mask: &[Vec<usize>], decode: bool) -> String {
    if mask.is_empty() {
        return text.to_string();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len());
    let mut index = 0usize;
    let mut group_index = 0usize;

    while index < chars.len() {
        let group = &mask[group_index % mask.len()];
        let block_len = group.len();

        if index + block_len > chars.len() {
            for &ch in &chars[index..] {
                result.push(ch);
            }
            break;
        }

        let block = &chars[index..index + block_len];

        if decode {
            let mut plain = vec!['\0'; block_len];
            for (cipher_position, &plain_position_1based) in group.iter().enumerate() {
                plain[plain_position_1based - 1] = block[cipher_position];
            }
            for ch in plain {
                result.push(ch);
            }
        } else {
            for &plain_position_1based in group {
                result.push(block[plain_position_1based - 1]);
            }
        }

        index += block_len;
        group_index += 1;
    }

    result
}

pub fn gronsfeld_cipher(input: &str, key: &str, decode: bool) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    let key_digits: Vec<i32> = key
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    if key_digits.is_empty() {
        return input.to_string();
    }

    fn shift_ascii(c: char, base: char, shift: i32) -> char {
        let idx = c as i32 - base as i32;
        let normalized = (idx + shift).rem_euclid(26);
        char::from_u32((base as u32) + normalized as u32).unwrap_or(c)
    }

    fn shift_in_alphabet(c: char, alphabet: &[char], shift: i32) -> Option<char> {
        let len = alphabet.len() as i32;
        alphabet
            .iter()
            .position(|&letter| letter == c)
            .map(|idx| alphabet[(idx as i32 + shift).rem_euclid(len) as usize])
    }

    let direction = if decode { -1 } else { 1 };
    let mut key_index = 0;

    input
        .chars()
        .map(|c| {
            let shift = key_digits[key_index % key_digits.len()] * direction;
            key_index += 1;

            if c.is_ascii_lowercase() {
                shift_ascii(c, 'a', shift)
            } else if c.is_ascii_uppercase() {
                shift_ascii(c, 'A', shift)
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_LOWER, shift) {
                mapped
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_UPPER, shift) {
                mapped
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_cipher(input: &str, key: &str, decode: bool) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    let key_chars: Vec<char> = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || RU_LOWER.contains(c) || RU_UPPER.contains(c))
        .collect();

    if key_chars.is_empty() {
        return input.to_string();
    }

    fn get_shift(c: char) -> i32 {
        if c.is_ascii_lowercase() {
            c as i32 - 'a' as i32
        } else if c.is_ascii_uppercase() {
            c as i32 - 'A' as i32
        } else if let Some(idx) = RU_LOWER.iter().position(|&l| l == c) {
            idx as i32
        } else if let Some(idx) = RU_UPPER.iter().position(|&l| l == c) {
            idx as i32
        } else {
            0
        }
    }

    fn shift_ascii(c: char, base: char, shift: i32) -> char {
        let idx = c as i32 - base as i32;
        let normalized = (idx + shift).rem_euclid(26);
        char::from_u32((base as u32) + normalized as u32).unwrap_or(c)
    }

    fn shift_in_alphabet(c: char, alphabet: &[char], shift: i32) -> Option<char> {
        let len = alphabet.len() as i32;
        alphabet
            .iter()
            .position(|&letter| letter == c)
            .map(|idx| alphabet[(idx as i32 + shift).rem_euclid(len) as usize])
    }

    let direction = if decode { -1 } else { 1 };
    let mut key_index = 0;

    input
        .chars()
        .map(|c| {
            let key_char = key_chars[key_index % key_chars.len()];
            let shift = get_shift(key_char) * direction;

            let result = if c.is_ascii_lowercase() {
                shift_ascii(c, 'a', shift)
            } else if c.is_ascii_uppercase() {
                shift_ascii(c, 'A', shift)
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_LOWER, shift) {
                mapped
            } else if let Some(mapped) = shift_in_alphabet(c, &RU_UPPER, shift) {
                mapped
            } else {
                c
            };

            if c.is_ascii_alphabetic() || RU_LOWER.contains(&c) || RU_UPPER.contains(&c) {
                key_index += 1;
            }

            result
        })
        .collect()
}

/// Xorshift32 PRNG with configurable shift constants
#[derive(Debug, Clone)]
pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    pub fn new(seed: u32) -> Self {
        Xorshift32 {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    pub fn next_with(&mut self, a: u32, b: u32, c: u32) -> u32 {
        let mut x = self.state;
        x ^= x.wrapping_shl(a);
        x ^= x.wrapping_shr(b);
        x ^= x.wrapping_shl(c);
        self.state = x;
        x
    }

    pub fn next_byte_with(&mut self, a: u32, b: u32, c: u32) -> u8 {
        (self.next_with(a, b, c) & 0xFF) as u8
    }
}

pub fn generate_gamma(length: usize, seed: u32, a: u32, b: u32, c: u32) -> Vec<u8> {
    let mut rng = Xorshift32::new(seed);
    (0..length).map(|_| rng.next_byte_with(a, b, c)).collect()
}

pub fn gamming_encrypt(plaintext: &str, seed: u32, a: u32, b: u32, c: u32) -> Vec<u8> {
    let bytes = plaintext.as_bytes();
    let gamma = generate_gamma(bytes.len(), seed, a, b, c);
    bytes.iter().zip(gamma).map(|(&b, g)| b ^ g).collect()
}

pub fn gamming_decrypt(ciphertext: &[u8], seed: u32, a: u32, b: u32, c: u32) -> String {
    let gamma = generate_gamma(ciphertext.len(), seed, a, b, c);
    let bytes: Vec<u8> = ciphertext.iter().zip(gamma).map(|(&b, g)| b ^ g).collect();
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}

pub fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    (0..cleaned.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex pair: {}", &cleaned[i..i + 2]))
        })
        .collect()
}

pub fn frequency_analysis(input: &str) -> Vec<(char, usize, f64)> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut total_chars = 0usize;

    for ch in input.chars() {
        if ch.is_alphabetic() {
            let ch_lower = ch.to_lowercase().next().unwrap_or(ch);
            *counts.entry(ch_lower).or_insert(0) += 1;
            total_chars += 1;
        }
    }

    if total_chars == 0 {
        return Vec::new();
    }

    let mut result: Vec<(char, usize, f64)> = counts
        .into_iter()
        .map(|(ch, count)| {
            let percentage = (count as f64 / total_chars as f64) * 100.0;
            (ch, count, percentage)
        })
        .collect();

    result.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    result
}

// Russian letter frequencies (as probabilities)
pub const RUSSIAN_FREQUENCIES: [(char, f64); 33] = [
    (' ', 0.175), ('О', 0.090), ('Е', 0.072), ('А', 0.062), ('И', 0.062),
    ('Н', 0.053), ('Т', 0.053), ('С', 0.045), ('Р', 0.040), ('В', 0.038),
    ('Л', 0.035), ('К', 0.028), ('М', 0.026), ('Д', 0.025), ('П', 0.023),
    ('У', 0.021), ('Я', 0.018), ('Ы', 0.016), ('З', 0.016), ('Ъ', 0.014),
    ('Б', 0.014), ('Г', 0.013), ('Ч', 0.012), ('Й', 0.010), ('Х', 0.009),
    ('Ж', 0.007), ('Ю', 0.006), ('Ш', 0.006), ('Ц', 0.004), ('Щ', 0.003),
    ('Э', 0.003), ('Ф', 0.002), ('Ё', 0.072),
];

// English letter frequencies (as probabilities)
pub const ENGLISH_FREQUENCIES: [(char, f64); 27] = [
    (' ', 0.180), ('E', 0.123), ('T', 0.096), ('A', 0.081), ('O', 0.079),
    ('N', 0.072), ('I', 0.071), ('S', 0.066), ('R', 0.060), ('H', 0.051),
    ('L', 0.040), ('D', 0.036), ('C', 0.032), ('U', 0.031), ('P', 0.023),
    ('F', 0.023), ('M', 0.022), ('W', 0.020), ('Y', 0.019), ('B', 0.016),
    ('G', 0.016), ('V', 0.009), ('K', 0.005), ('Q', 0.002), ('X', 0.002),
    ('J', 0.001), ('Z', 0.001),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Russian,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Russian
    }
}

pub fn frequency_decrypt(ciphertext: &str, language: Language) -> String {
    if ciphertext.is_empty() {
        return String::new();
    }

    let freq_table: &[(char, f64)] = match language {
        Language::Russian => &RUSSIAN_FREQUENCIES,
        Language::English => &ENGLISH_FREQUENCIES,
    };

    let cipher_freq = frequency_analysis(ciphertext);

    if cipher_freq.is_empty() {
        return ciphertext.to_string();
    }

    let target_letters: Vec<char> = freq_table
        .iter()
        .filter_map(|(ch, _)| {
            if *ch == ' ' {
                None
            } else {
                Some(*ch)
            }
        })
        .collect();

    let mut mapping: HashMap<char, char> = HashMap::new();
    for (i, (cipher_char, _, _)) in cipher_freq.iter().enumerate() {
        if i < target_letters.len() {
            mapping.insert(*cipher_char, target_letters[i]);
        }
    }

    ciphertext
        .chars()
        .map(|c| {
            if c.is_whitespace() {
                c
            } else {
                let lookup = c.to_lowercase().next().unwrap_or(c);

                if let Some(&plain_char) = mapping.get(&lookup) {
                    if c.is_ascii_alphabetic() {
                        if c.is_uppercase() {
                            plain_char.to_ascii_uppercase()
                        } else {
                            plain_char.to_ascii_lowercase()
                        }
                    } else if c.is_uppercase() {
                        plain_char.to_uppercase().next().unwrap_or(plain_char)
                    } else if c.is_lowercase() {
                        plain_char.to_lowercase().next().unwrap_or(plain_char)
                    } else {
                        plain_char
                    }
                } else {
                    c
                }
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct FrequencySubstitution {
    pub entries: Vec<(char, char)>,
    mapping: HashMap<char, char>,
}

impl FrequencySubstitution {
    pub fn new(ciphertext: &str, language: Language) -> Self {
        let freq_table: &[(char, f64)] = match language {
            Language::Russian => &RUSSIAN_FREQUENCIES,
            Language::English => &ENGLISH_FREQUENCIES,
        };

        let cipher_freq = frequency_analysis(ciphertext);

        let target_letters: Vec<char> = freq_table
            .iter()
            .filter_map(|(ch, _)| if *ch == ' ' { None } else { Some(*ch) })
            .collect();

        let mut mapping = HashMap::new();
        let mut entries = Vec::new();
        for (i, (cipher_char, _, _)) in cipher_freq.iter().enumerate() {
            if i < target_letters.len() {
                let plain_lower = target_letters[i].to_lowercase().next().unwrap_or(target_letters[i]);
                mapping.insert(*cipher_char, plain_lower);
                entries.push((*cipher_char, plain_lower));
            }
        }

        FrequencySubstitution { entries, mapping }
    }

    pub fn swap(&mut self, plain_a: char, plain_b: char) {
        let plain_a_lower = plain_a.to_lowercase().next().unwrap_or(plain_a);
        let plain_b_lower = plain_b.to_lowercase().next().unwrap_or(plain_b);

        let rev: HashMap<char, char> =
            self.mapping.iter().map(|(&k, &v)| (v, k)).collect();

        if let Some(&cipher_a) = rev.get(&plain_a_lower) {
            if let Some(&cipher_b) = rev.get(&plain_b_lower) {
                self.mapping.insert(cipher_a, plain_b_lower);
                self.mapping.insert(cipher_b, plain_a_lower);
                for entry in self.entries.iter_mut() {
                    if entry.0 == cipher_a {
                        entry.1 = plain_b_lower;
                    } else if entry.0 == cipher_b {
                        entry.1 = plain_a_lower;
                    }
                }
            }
        }
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    c
                } else {
                    let lookup = c.to_lowercase().next().unwrap_or(c);
                    if let Some(&plain_char) = self.mapping.get(&lookup) {
                        if c.is_ascii_alphabetic() {
                            if c.is_uppercase() {
                                plain_char.to_ascii_uppercase()
                            } else {
                                plain_char.to_ascii_lowercase()
                            }
                        } else if c.is_uppercase() {
                            plain_char.to_uppercase().next().unwrap_or(plain_char)
                        } else if c.is_lowercase() {
                            plain_char.to_lowercase().next().unwrap_or(plain_char)
                        } else {
                            plain_char
                        }
                    } else {
                        c
                    }
                }
            })
            .collect()
    }
}
