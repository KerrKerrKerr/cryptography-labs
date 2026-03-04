use iced::Task;
use iced::Theme;
use iced::{
    Element,
    Length,
    widget::{Text, button, column, container, responsive, row, text_input},
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Message {
    Atbash,
    Ceasar,
    Rishelau,
    AtbashInput(String),
    AtbashOutputIgnored(String),
    CeasarInput(String),
    CeasarShift(String),
    CeasarOutputIgnored(String),
    RishelauInput(String),
    RishelauOutput(String),
    RishelauMask(String),
}

#[derive(Default)]
enum RishelauLastEdited {
    #[default]
    Input,
    Output,
}

fn atbash_cipher(input: &str) -> String {
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

fn caesar_cipher(input: &str, shift: i32) -> String {
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

fn parse_rishelau_mask(mask: &str) -> Result<Vec<Vec<usize>>, String> {
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

fn rishelau_cipher(text: &str, mask: &[Vec<usize>], decode: bool) -> String {
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

//rough transcriptions from Russian
#[derive(Default)]
pub enum Ciphers {
    #[default]
    ATBASH,
    CEASAR,
    RISHELAU,
}

#[derive(Default)]
pub struct AppState {
    cipher_selected: Ciphers,
    atbash_input: String,
    ceasar_input: String,
    ceasar_shift: String,
    rishelau_input: String,
    rishelau_output: String,
    rishelau_mask: String,
    rishelau_error: String,
    rishelau_last_edited: RishelauLastEdited,
    atbash_output: String,
    ceasar_output: String,
}

impl AppState {
    fn recalc_rishelau(&mut self) {
        match parse_rishelau_mask(&self.rishelau_mask) {
            Ok(mask) => {
                self.rishelau_error.clear();
                if mask.is_empty() {
                    match self.rishelau_last_edited {
                        RishelauLastEdited::Input => {
                            self.rishelau_output = self.rishelau_input.clone();
                        }
                        RishelauLastEdited::Output => {
                            self.rishelau_input = self.rishelau_output.clone();
                        }
                    }
                    return;
                }

                match self.rishelau_last_edited {
                    RishelauLastEdited::Input => {
                        self.rishelau_output = rishelau_cipher(&self.rishelau_input, &mask, false);
                    }
                    RishelauLastEdited::Output => {
                        self.rishelau_input = rishelau_cipher(&self.rishelau_output, &mask, true);
                    }
                }
            }
            Err(error) => {
                self.rishelau_error = error;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> { 
        responsive(|size| {
            let content = column![
                self.draw_cipher_selector(),
                match self.cipher_selected {
                    Ciphers::ATBASH => self.draw_atbash(),
                    Ciphers::CEASAR => self.draw_ceasar(),
                    Ciphers::RISHELAU => self.draw_rishelau(),
                }
            ]
            .spacing(16)
            .padding(16)
            .max_width(if size.width < 900.0 { 760 } else { 980 });

            container(content)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
        }).into()
    }
    
    fn draw_atbash(&self) -> Element<'_, Message> {
        responsive(|size| {
            let input_atbash = text_input("Type plain text or cipher text", &self.atbash_input)
            .on_input(Message::AtbashInput)
            .padding(10)
            .size(20)
            .width(Length::Fill);

            let input_block = column![
                Text::new("Input"),
                input_atbash,
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let output_block = column![
                Text::new("Output (selectable)"),
                text_input("Output", &self.atbash_output)
                    .on_input(Message::AtbashOutputIgnored)
                    .padding(10)
                    .size(20)
                    .width(Length::Fill),
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let io_layout: Element<'_, Message> = if size.width < 760.0 {
                column![input_block, output_block]
                    .spacing(12)
                    .width(Length::Fill)
                    .into()
            } else {
                row![input_block, output_block]
                    .spacing(16)
                    .width(Length::Fill)
                    .into()
            };

            column![
                Text::new("Atbash Cipher").size(28),
                Text::new("Atbash is symmetric: to decode, just paste the ciphered text into the input.").size(16),
                io_layout,
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        }).into()
    }

    fn draw_ceasar(&self) -> Element<'_, Message> {
        responsive(|size| {
            let ceasar_input = text_input("Type plain text or cipher text, to decipher enter the cipher text and negative shift", &self.ceasar_input)
                .on_input(Message::CeasarInput)
                .padding(10)
                .size(20)
                .width(Length::Fill);

            let ceasar_shift = text_input("Shift (e.g. 3 or -3)", &self.ceasar_shift)
                .on_input(Message::CeasarShift)
                .padding(10)
                .size(20)
                .width(Length::Fill);

            let left_block = column![
                Text::new("Input"),
                ceasar_input,
                Text::new("Shift"),
                ceasar_shift,
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let right_block = column![
                Text::new("Output (selectable)"),
                text_input("Output", &self.ceasar_output)
                    .on_input(Message::CeasarOutputIgnored)
                    .padding(10)
                    .size(20)
                    .width(Length::Fill),
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let io_layout: Element<'_, Message> = if size.width < 760.0 {
                column![left_block, right_block]
                    .spacing(12)
                    .width(Length::Fill)
                    .into()
            } else {
                row![left_block, right_block]
                    .spacing(16)
                    .width(Length::Fill)
                    .into()
            };

            column![
                Text::new("Ceasar Cipher").size(28),
                Text::new("Latin and Cyrillic are shifted separately with the same shift value.").size(16),
                io_layout,
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        }).into()
    }

    fn draw_rishelau(&self) -> Element<'_, Message> {
        responsive(|size| {
            let input_len = self.rishelau_input.chars().count();
            let mask_input = text_input("Mask, e.g. 3 1 2 | 2 1", &self.rishelau_mask)
                .on_input(Message::RishelauMask)
                .padding(10)
                .size(20)
                .width(Length::Fill);

            let mask_status = if self.rishelau_error.is_empty() {
                if self.rishelau_mask.trim().is_empty() {
                    Text::new("Mask is optional. Empty mask means no transformation.")
                } else {
                    Text::new("Mask is valid.")
                }
            } else {
                Text::new(format!("Mask error: {}", self.rishelau_error))
            };

            let left_block = column![
                Text::new("Input (editable, for encoding)"),
                text_input("Type plain text", &self.rishelau_input)
                    .on_input(Message::RishelauInput)
                    .padding(10)
                    .size(20)
                    .width(Length::Fill),
                Text::new(format!("Input length: {} characters", input_len)),
                Text::new("Mask"),
                mask_input,
                mask_status,
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let right_block = column![
                Text::new("Output (editable, for decoding)"),
                text_input("Type cipher text", &self.rishelau_output)
                    .on_input(Message::RishelauOutput)
                    .padding(10)
                    .size(20)
                    .width(Length::Fill),
                Text::new("No separators are inserted between cipher parts."),
            ]
            .spacing(8)
            .width(Length::FillPortion(1));

            let io_layout: Element<'_, Message> = if size.width < 760.0 {
                column![left_block, right_block]
                    .spacing(12)
                    .width(Length::Fill)
                    .into()
            } else {
                row![left_block, right_block]
                    .spacing(16)
                    .width(Length::Fill)
                    .into()
            };

            column![
                Text::new("Rishelau Cipher").size(28),
                Text::new("Edit either side: input encodes to output, output decodes back to input.").size(16),
                io_layout,
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        }).into()
    }

    fn draw_cipher_selector(&self) -> Element<'_, Message> {
        row![
            Text::new("Select cipher:"),
            button("Atbash").on_press(Message::Atbash),
            button("Ceasar").on_press(Message::Ceasar),
            button("Rishelau").on_press(Message::Rishelau),
        ].spacing(5).padding(5).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Atbash => {
                self.cipher_selected = Ciphers::ATBASH;
                Task::none()
            },
            Message::Ceasar => {
                self.cipher_selected = Ciphers::CEASAR;
                Task::none()
            },
            Message::Rishelau => {
                self.cipher_selected = Ciphers::RISHELAU;
                Task::none()
            },
            Message::AtbashInput(input) => {
                self.atbash_input = input;
                self.atbash_output = atbash_cipher(&self.atbash_input);
                Task::none()
            },
            Message::AtbashOutputIgnored(_input) => Task::none(),
            Message::CeasarInput(input) => {
                self.ceasar_input = input;
                let shift = self.ceasar_shift.trim().parse::<i32>().unwrap_or(0);
                self.ceasar_output = caesar_cipher(&self.ceasar_input, shift);
                Task::none()
            },
            Message::CeasarShift(shift) => {
                self.ceasar_shift = shift;
                let shift = self.ceasar_shift.trim().parse::<i32>().unwrap_or(0);
                self.ceasar_output = caesar_cipher(&self.ceasar_input, shift);
                Task::none()
            },
            Message::CeasarOutputIgnored(_input) => Task::none(),
            Message::RishelauInput(input) => {
                self.rishelau_input = input;
                self.rishelau_last_edited = RishelauLastEdited::Input;
                self.recalc_rishelau();
                Task::none()
            },
            Message::RishelauOutput(output) => {
                self.rishelau_output = output;
                self.rishelau_last_edited = RishelauLastEdited::Output;
                self.recalc_rishelau();
                Task::none()
            },
            Message::RishelauMask(mask) => {
                self.rishelau_mask = mask;
                self.recalc_rishelau();
                Task::none()
            },
        }
    }
}





fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(Theme::Light)
        .window_size((800, 600))
        .resizable(true)
        .run()
}
