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