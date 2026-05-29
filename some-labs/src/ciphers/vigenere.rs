pub fn vigenere_cipher(input: &str, key: &str, decrypt: bool) -> String {
    const RU_LOWER: [char; 33] = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
        'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];
    const RU_UPPER: [char; 33] = [
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
        'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
    ];

    fn get_char_index(c: char) -> Option<(usize, bool, bool)> {
        // Returns (index, is_russian, is_uppercase)
        if c.is_ascii_lowercase() {
            return Some((c as usize - 'a' as usize, false, false));
        } else if c.is_ascii_uppercase() {
            return Some((c as usize - 'A' as usize, false, true));
        } else if let Some(idx) = RU_LOWER.iter().position(|&letter| letter == c) {
            return Some((idx, true, false));
        } else if let Some(idx) = RU_UPPER.iter().position(|&letter| letter == c) {
            return Some((idx, true, true));
        }
        None
    }

    fn index_from_key_char(c: char) -> Option<usize> {
        if c.is_ascii_lowercase() {
            return Some(c as usize - 'a' as usize);
        } else if c.is_ascii_uppercase() {
            return Some(c as usize - 'A' as usize);
        } else if let Some(idx) = RU_LOWER.iter().position(|&letter| letter == c) {
            return Some(idx);
        } else if let Some(idx) = RU_UPPER.iter().position(|&letter| letter == c) {
            return Some(idx);
        }
        None
    }

    let key_indices: Vec<usize> = key.chars().filter_map(index_from_key_char).collect();
    if key_indices.is_empty() {
        return input.to_string();
    }

    let mut key_pos = 0;
    let mut letter_count = 0;

    input
        .chars()
        .map(|c| {
            if let Some((idx, is_ru, is_upper)) = get_char_index(c) {
                let shift = key_indices[key_pos % key_indices.len()];
                let alphabet_size = if is_ru { 33 } else { 26 };
                let shifted = if decrypt {
                    (idx as i32 - shift as i32 + alphabet_size as i32) % alphabet_size as i32
                } else {
                    (idx as i32 + shift as i32) % alphabet_size as i32
                };

                let result = if is_ru {
                    if is_upper {
                        RU_UPPER[shifted as usize]
                    } else {
                        RU_LOWER[shifted as usize]
                    }
                } else {
                    if is_upper {
                        char::from_u32(shifted as u32 + 'A' as u32).unwrap_or(c)
                    } else {
                        char::from_u32(shifted as u32 + 'a' as u32).unwrap_or(c)
                    }
                };

                letter_count += 1;
                if letter_count % 10 == 0 {
                    key_pos += 1;
                }

                result
            } else {
                c
            }
        })
        .collect()
}