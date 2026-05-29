use std::collections::HashSet;

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

    // Фильтруем ключ, оставляя только буквы
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
            // Находим сдвиг для текущего символа ключа
            let key_char = key_chars[key_index % key_chars.len()];
            let shift = get_shift(key_char) * direction;

            // Сдвигаем символ текста
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

            // Увеличиваем индекс ключа только если символ был зашифрован
            if c.is_ascii_alphabetic() || RU_LOWER.contains(&c) || RU_UPPER.contains(&c) {
                key_index += 1;
            }

            result
        })
        .collect()
}

pub fn frequency_analysis(input: &str) -> Vec<(char, usize, f64)> {
    use std::collections::HashMap;
    
    if input.is_empty() {
        return Vec::new();
    }
    
    // Count character frequencies
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut total_chars = 0usize;
    
    for ch in input.chars() {
        // Include only letters; ignore punctuation, control chars and spaces
        if ch.is_alphabetic() {
            let ch_lower = ch.to_lowercase().next().unwrap_or(ch);
            *counts.entry(ch_lower).or_insert(0) += 1;
            total_chars += 1;
        }
    }
    
    if total_chars == 0 {
        return Vec::new();
    }
    
    // Convert to vector with percentages and sort by count (descending)
    let mut result: Vec<(char, usize, f64)> = counts
        .into_iter()
        .map(|(ch, count)| {
            let percentage = (count as f64 / total_chars as f64) * 100.0;
            (ch, count, percentage)
        })
        .collect();
    
    // Sort by count descending, then by character for stable ordering
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

    // Get frequency table for the selected language
    let freq_table: &[(char, f64)] = match language {
        Language::Russian => &RUSSIAN_FREQUENCIES,
        Language::English => &ENGLISH_FREQUENCIES,
    };

    // Analyze ciphertext frequencies
    let cipher_freq = frequency_analysis(ciphertext);
    
    if cipher_freq.is_empty() {
        return ciphertext.to_string();
    }

    // Create mapping from ciphertext letters to plaintext letters
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

    let mut mapping: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    for (i, (cipher_char, _, _)) in cipher_freq.iter().enumerate() {
        if i < target_letters.len() {
            mapping.insert(*cipher_char, target_letters[i]);
        }
    }

    // Apply mapping to decrypt, keep whitespace unchanged
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