/// Supported languages for frequency analysis decryption.
#[derive(Debug, Clone, Copy, Default)]
pub enum Language {
    #[default]
    Russian,
    English,
}

/// Performs frequency analysis on the input text.
/// Returns a list of (character, count, percentage) sorted by count descending.
pub fn frequency_analysis(input: &str) -> Vec<(char, usize, f64)> {
    let mut freq: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    let mut total: usize = 0;

    for c in input.chars() {
        if c.is_alphabetic() {
            let lower = c.to_lowercase().next().unwrap_or(c);
            *freq.entry(lower).or_insert(0) += 1;
            total += 1;
        }
    }

    let mut result: Vec<(char, usize, f64)> = freq
        .into_iter()
        .map(|(ch, count)| {
            let pct = if total > 0 { (count as f64 / total as f64) * 100.0 } else { 0.0 };
            (ch, count, (pct * 100.0) / 100.0)
        })
        .collect();

    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}

/// Russian letter frequency order (most common to least common).
const RUSSIAN_FREQ_ORDER: &[char] = &[
    'о', 'а', 'е', 'и', 'н', 'т', 'с', 'р', 'в', 'л',
    'к', 'м', 'д', 'п', 'у', 'ы', 'я', 'ж', 'б', 'г',
    'ь', 'з', 'й', 'х', 'ш', 'щ', 'ц', 'ч', 'ф', 'ю',
    'ъ', 'э', 'ё',
];

/// English letter frequency order (most common to least common).
const ENGLISH_FREQ_ORDER: &[char] = &[
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd',
    'l', 'c', 'u', 'm', 'w', 'f', 'g', 'y', 'p', 'b',
    'v', 'k', 'j', 'x', 'q', 'z',
];

/// A substitution map that holds cipher→plain character mappings.
#[derive(Debug, Clone, Default)]
pub struct FrequencySubstitution {
    pub map: std::collections::HashMap<char, char>,
}

impl FrequencySubstitution {
    /// Apply this substitution to the given text, preserving case.
    pub fn apply(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            if c.is_alphabetic() {
                let lower = c.to_lowercase().next().unwrap_or(c);
                if let Some(&replacement) = self.map.get(&lower) {
                    if c.is_uppercase() {
                        result.push(replacement.to_uppercase().next().unwrap_or(replacement));
                    } else {
                        result.push(replacement);
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}

/// Automatically build a substitution map by matching the most frequent letters
/// in the ciphertext to the most frequent letters in the target language.
pub fn auto_substitute(input: &str, language: Language) -> FrequencySubstitution {
    let freq = frequency_analysis(input);
    let target_order = match language {
        Language::Russian => RUSSIAN_FREQ_ORDER,
        Language::English => ENGLISH_FREQ_ORDER,
    };

    if freq.is_empty() || target_order.is_empty() {
        return FrequencySubstitution::default();
    }

    let mut substitution = FrequencySubstitution::default();
    for (i, &(cipher_char, _, _)) in freq.iter().enumerate() {
        if i < target_order.len() {
            substitution.map.insert(cipher_char, target_order[i]);
        }
    }

    substitution
}

/// Decrypts the input using frequency analysis-based substitution.
/// Maps the most frequent letter in the ciphertext to the most frequent in the target language, etc.
pub fn frequency_decrypt(input: &str, language: Language) -> String {
    auto_substitute(input, language).apply(input)
}
