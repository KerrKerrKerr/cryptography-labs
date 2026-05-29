/// Rishelau (Richelieu) cipher implementation.
/// Uses a grid-based substitution with a numeric mask/key.
pub fn rishelau_cipher(input: &str, mask: &Vec<Vec<i32>>, decrypt: bool) -> String {
    let mut result = String::with_capacity(input.len());
    let rows = mask.len();
    let cols = if rows > 0 { mask[0].len() } else { 0 };
    let grid_size = rows * cols;

    if grid_size == 0 {
        return input.to_string();
    }

    // Build alphabet: Russian letters
    let alphabet: Vec<char> = [
        'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и',
        'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т',
        'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
        'э', 'ю', 'я',
    ].iter().chain(&['А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И',
        'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т',
        'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь',
        'Э', 'Ю', 'Я']).cloned().collect();

    for c in input.chars() {
        if let Some(pos) = alphabet.iter().position(|&letter| letter == c) {
            let (mut r, mut col) = (pos / grid_size, pos % grid_size);
            if decrypt {
                // Decrypt: reverse the mask operation
                let mr = r % rows;
                let mc = col % cols;
                r -= mask[mr][mc] as usize;
                col -= mask[mr][mc] as usize;
            } else {
                // Encrypt: apply the mask
                let mr = r % rows;
                let mc = col % cols;
                r += mask[mr][mc] as usize;
                col += mask[mr][mc] as usize;
            }
            let new_pos = r * grid_size + col;
            if new_pos < alphabet.len() {
                result.push(alphabet[new_pos]);
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Parses a mask string like "1,2,3|4,5,6" into a 2D vector.
/// Returns an error String if the format is invalid.
pub fn parse_rishelau_mask(mask_str: &str) -> Result<Vec<Vec<i32>>, String> {
    let mut grid = Vec::new();
    let rows: Vec<&str> = mask_str.split('|').collect();

    for (i, row_str) in rows.iter().enumerate() {
        let row_str = row_str.trim();
        if row_str.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        for (j, val_str) in row_str.split(',').enumerate() {
            match val_str.trim().parse::<i32>() {
                Ok(val) => row.push(val),
                Err(_) => {
                    return Err(format!("Invalid number at row {}, column {}: '{}'", i, j, val_str.trim()));
                }
            }
        }
        if !row.is_empty() {
            grid.push(row);
        }
    }

    // Validate all rows have the same length
    if !grid.is_empty() {
        let first_len = grid[0].len();
        for (i, row) in grid.iter().enumerate() {
            if row.len() != first_len {
                return Err(format!("Row {} has {} columns, expected {}", i, row.len(), first_len));
            }
        }
    }

    Ok(grid)
}