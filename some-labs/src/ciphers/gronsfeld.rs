pub fn gronsfeld_cipher(input: &str, key: &str, decrypt: bool) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    fn map_from_alphabet(c: char, alphabet: &[char]) -> Option<usize> {
        alphabet.iter().position(|&letter| letter == c)
    }

    let digits: Vec<i32> = key.chars().filter_map(|c| c.to_digit(10)).map(|d| d as i32).collect();
    if digits.is_empty() {
        return input.to_string();
    }

    let mut key_idx = 0;
    let mut letter_count = 0;

    input
        .chars()
        .map(|c| {
            let is_letter = c.is_ascii_alphabetic()
                || map_from_alphabet(c, &RU_LOWER).is_some()
                || map_from_alphabet(c, &RU_UPPER).is_some();

            if !is_letter {
                return c;
            }

            let shift = digits[key_idx % digits.len()];

            let transformed = if c.is_ascii_lowercase() {
                let offset = c as i32 - 'a' as i32;
                let shifted = if decrypt {
                    (offset - shift + 26) % 26
                } else {
                    (offset + shift) % 26
                };
                char::from_u32(shifted as u32 + 'a' as u32).unwrap_or(c)
            } else if c.is_ascii_uppercase() {
                let offset = c as i32 - 'A' as i32;
                let shifted = if decrypt {
                    (offset - shift + 26) % 26
                } else {
                    (offset + shift) % 26
                };
                char::from_u32(shifted as u32 + 'A' as u32).unwrap_or(c)
            } else if let Some(idx) = map_from_alphabet(c, &RU_LOWER) {
                let shifted = if decrypt {
                    (idx as i32 - shift + 33) % 33
                } else {
                    (idx as i32 + shift) % 33
                };
                RU_LOWER[shifted as usize]
            } else if let Some(idx) = map_from_alphabet(c, &RU_UPPER) {
                let shifted = if decrypt {
                    (idx as i32 - shift + 33) % 33
                } else {
                    (idx as i32 + shift) % 33
                };
                RU_UPPER[shifted as usize]
            } else {
                c
            };

            letter_count += 1;
            if letter_count % 10 == 0 {
                key_idx += 1;
            }

            transformed
        })
        .collect()
}