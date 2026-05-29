/// Caesar cipher implementation supporting both English and Russian alphabets.
/// Automatically detects alphabet based on input content.
pub fn caesar_cipher(input: &str, shift: i32) -> String {
    let mut result = String::with_capacity(input.len());
    let mut has_russian = false;

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            has_russian = false;
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as i32 - base as i32 + shift).rem_euclid(26) + base as i32) as u8 as char;
            result.push(shifted);
        } else if ('а'..='я').contains(&c) || ('А'..='Я').contains(&c) || c == 'ё' || c == 'Ё' {
            has_russian = true;
            let shifted = caesar_russian_char(c, shift);
            result.push(shifted);
        } else {
            result.push(c);
        }
    }

    result
}

fn caesar_russian_char(c: char, shift: i32) -> char {
    const RUSSIAN_LOWER: &[char] = &[
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и',
        'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т',
        'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
        'э', 'ю', 'я',
    ];
    const RUSSIAN_UPPER: &[char] = &[
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И',
        'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т',
        'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь',
        'Э', 'Ю', 'Я',
    ];

    let alphabet = if RUSSIAN_UPPER.contains(&c) {
        RUSSIAN_UPPER
    } else {
        RUSSIAN_LOWER
    };

    if let Some(pos) = alphabet.iter().position(|&letter| letter == c) {
        let new_pos = (pos as i32 + shift).rem_euclid(33) as usize;
        alphabet[new_pos]
    } else {
        c
    }
}