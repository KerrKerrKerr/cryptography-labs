use iced::{Element, Length, widget::{Text, button, column, responsive, row, text_input, scrollable, container}};
use iced::widget::text::LineHeight;

use crate::{Message, Language};

pub fn draw_cipher_selector<'a>() -> Element<'a, Message> {
    row![
        Text::new("Select cipher:"),
        button("Atbash").on_press(Message::Atbash),
        button("Ceasar").on_press(Message::Ceasar),
        button("Rishelau").on_press(Message::Rishelau),
        button("Gronsfeld").on_press(Message::Gronsfeld),
        button("Vigenere").on_press(Message::Vigenere),
        button("Frequency Analysis").on_press(Message::FrequencyAnalysis),
    ]
    .spacing(5)
    .padding(5)
    .into()
}

pub fn draw_atbash<'a>(input: &'a str, output: &'a str) -> Element<'a, Message> {
    responsive(move |size| {
        let input_atbash = text_input("Type plain text or cipher text", input)
            .on_input(Message::AtbashInput)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let input_block = column![Text::new("Input"), input_atbash]
            .spacing(8)
            .width(Length::FillPortion(1));

        let output_block = column![
            Text::new("Output (selectable)"),
            text_input("Output", output)
                .on_input(Message::AtbashOutputIgnored)
                .padding(10)
                .size(20)
                .width(Length::Fill),
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let io_layout: Element<'_, Message> = if size.width < 760.0 {
            column![input_block, output_block].spacing(12).width(Length::Fill).into()
        } else {
            row![input_block, output_block].spacing(16).width(Length::Fill).into()
        };

        column![
            Text::new("Atbash Cipher").size(28),
            Text::new("Atbash is symmetric: to decode, just paste the ciphered text into the input.").size(16),
            io_layout,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}

pub fn draw_ceasar<'a>(input: &'a str, shift: &'a str, output: &'a str) -> Element<'a, Message> {
    responsive(move |size| {
        let ceasar_input = text_input("Type plain text or cipher text, to decipher enter the cipher text and negative shift", input)
            .on_input(Message::CeasarInput)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let ceasar_shift = text_input("Shift (e.g. 3 or -3)", shift)
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
            text_input("Output", output)
                .on_input(Message::CeasarOutputIgnored)
                .padding(10)
                .size(20)
                .width(Length::Fill),
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let io_layout: Element<'_, Message> = if size.width < 760.0 {
            column![left_block, right_block].spacing(12).width(Length::Fill).into()
        } else {
            row![left_block, right_block].spacing(16).width(Length::Fill).into()
        };

        column![
            Text::new("Ceasar Cipher").size(28),
            Text::new("Latin and Cyrillic are shifted separately with the same shift value.").size(16),
            io_layout,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}

pub fn draw_rishelau<'a>(input: &'a str, output: &'a str, mask: &'a str, error: &'a str) -> Element<'a, Message> {
    responsive(move |size| {
        let input_len = input.chars().count();

        let mask_input = text_input("Mask, e.g. 3 1 2 | 2 1", mask)
            .on_input(Message::RishelauMask)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let mask_status = if error.is_empty() {
            if mask.trim().is_empty() {
                Text::new("Mask is optional. Empty mask means no transformation.")
            } else {
                Text::new("Mask is valid.")
            }
        } else {
            Text::new(format!("Mask error: {}", error))
        };

        let left_block = column![
            Text::new("Input (editable, for encoding)"),
            text_input("Type plain text", input)
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
            text_input("Type cipher text", output)
                .on_input(Message::RishelauOutput)
                .padding(10)
                .size(20)
                .width(Length::Fill),
            Text::new("No separators are inserted between cipher parts."),
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let io_layout: Element<'_, Message> = if size.width < 760.0 {
            column![left_block, right_block].spacing(12).width(Length::Fill).into()
        } else {
            row![left_block, right_block].spacing(16).width(Length::Fill).into()
        };

        column![
            Text::new("Rishelau Cipher").size(28),
            Text::new("Edit either side: input encodes to output, output decodes back to input.").size(16),
            io_layout,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}

pub fn draw_gronsfeld<'a>(input: &'a str, key: &'a str, output: &'a str) -> Element<'a, Message> {
    responsive(move |size| {
        let gronsfeld_input = text_input("Type plain text or cipher text", input)
            .on_input(Message::GronsfeldInput)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let gronsfeld_key = text_input("Key (digits only, e.g. 12345)", key)
            .on_input(Message::GronsfeldKey)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let left_block = column![
            Text::new("Input"),
            gronsfeld_input,
            Text::new("Key"),
            gronsfeld_key,
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let right_block = column![
            Text::new("Output (selectable)"),
            text_input("Output", output)
                .on_input(Message::GronsfeldOutputIgnored)
                .padding(10)
                .size(20)
                .width(Length::Fill),
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let io_layout: Element<'_, Message> = if size.width < 760.0 {
            column![left_block, right_block].spacing(12).width(Length::Fill).into()
        } else {
            row![left_block, right_block].spacing(16).width(Length::Fill).into()
        };

        column![
            Text::new("Gronsfeld Cipher").size(28),
            Text::new("Enter text and a numeric key. To decrypt, enter cipher text and the same key.").size(16),
            io_layout,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}

pub fn draw_vigenere<'a>(input: &'a str, key: &'a str, output: &'a str) -> Element<'a, Message> {
    responsive(move |size| {
        let vigenere_input = text_input("Type plain text or cipher text", input)
            .on_input(Message::VigenereInput)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let vigenere_key = text_input("Key (letters only, e.g. KEYWORD)", key)
            .on_input(Message::VigenereKey)
            .padding(10)
            .size(20)
            .width(Length::Fill);

        let left_block = column![
            Text::new("Input"),
            vigenere_input,
            Text::new("Key"),
            vigenere_key,
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let right_block = column![
            Text::new("Output (selectable)"),
            text_input("Output", output)
                .on_input(Message::VigenereOutputIgnored)
                .padding(10)
                .size(20)
                .width(Length::Fill),
        ]
        .spacing(8)
        .width(Length::FillPortion(1));

        let io_layout: Element<'_, Message> = if size.width < 760.0 {
            column![left_block, right_block].spacing(12).width(Length::Fill).into()
        } else {
            row![left_block, right_block].spacing(16).width(Length::Fill).into()
        };

        column![
            Text::new("Vigenere Cipher").size(28),
            Text::new("Enter text and a keyword. To decrypt, enter cipher text and the same key.").size(16),
            io_layout,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}

pub fn draw_frequency_analysis<'a>(
    input: &'a str,
    result: &'a [(char, usize, f64)],
    error: &'a str,
    decrypted: &'a str,
    selected_lang: Language,
) -> Element<'a, Message> {
    responsive(move |_size| {
        // Input area with buttons
        let input_area = container(
            text_input("Enter text for frequency analysis", input)
                .on_input(Message::FreqAnalysisInput)
                .line_height(LineHeight::Relative(1.2))
                .padding(10)
                .size(20)
                .width(Length::Fill)
        )
        .height(Length::Fixed(180.0))
        .width(Length::Fill);

        let buttons_row = row![
            button("Paste from clipboard").on_press(Message::FreqAnalysisPaste),
            button("Load from file").on_press(Message::FreqAnalysisLoadFile),
        ]
        .spacing(8);

        // Decryption buttons
        let decrypt_row = row![
            Text::new("Try decrypt:").size(16),
            button("Russian").on_press(Message::FreqAnalysisDecrypt(Language::Russian)),
            button("English").on_press(Message::FreqAnalysisDecrypt(Language::English)),
        ]
        .spacing(8);

        // Show selected language
        let lang_text = format!("Selected language: {:?}", selected_lang);

        // Build frequency table
        let mut table_content: iced::widget::Column<'_, Message> = column![];
        
        if !result.is_empty() {
            // Header row
            let header = row![
                container(Text::new("Symbol").size(16)).width(Length::Fixed(80.0)),
                container(Text::new("Count").size(16)).width(Length::Fixed(80.0)),
                container(Text::new("Frequency %").size(16)).width(Length::Fixed(100.0)),
                container(Text::new("Bar").size(16)).width(Length::Fill),
            ]
            .spacing(10)
            .padding(5);
            
            table_content = table_content.push(header);
            
            // Separator line
            let separator = container(
                iced::widget::Space::new()
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
            )
                .style(|_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.7, 0.7, 0.7))),
                    ..Default::default()
                });
            table_content = table_content.push(separator);

            // Find max count for bar scaling
            let max_count = result.iter().map(|(_, count, _)| *count).max().unwrap_or(1);

            // Data rows
            for (ch, count, percentage) in result {
                let bar_width = (*count as f32 / max_count as f32) * 200.0;
                let bar = container(
                    iced::widget::Space::new()
                        .width(Length::Fixed(bar_width))
                        .height(Length::Fixed(16.0))
                )
                    .style(|_theme: &iced::Theme| iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.3, 0.6, 0.9))),
                        ..Default::default()
                    });

                let row_data = row![
                    container(Text::new(format!("'{}'", ch)).size(14)).width(Length::Fixed(80.0)),
                    container(Text::new(count.to_string()).size(14)).width(Length::Fixed(80.0)),
                    container(Text::new(format!("{:.2}%", percentage)).size(14)).width(Length::Fixed(100.0)),
                    container(bar).width(Length::Fill),
                ]
                .spacing(10)
                .padding(3);

                table_content = table_content.push(row_data);
            }
        }

        let scrollable_table = scrollable(table_content)
            .height(Length::Fixed(250.0))
            .width(Length::Fill);

        let error_text = if error.is_empty() {
            Text::new("")
        } else {
            Text::new(error).size(14)
        };

        // Decrypted text area (read-only, multiline display)
        let decrypted_area = if !decrypted.is_empty() {
            column![
                Text::new("Decrypted text (frequency-based):").size(18),
                container(
                    scrollable(Text::new(decrypted).size(16))
                        .height(Length::Fill)
                )
                .height(Length::Fixed(180.0))
                .width(Length::Fill)
            ]
            .spacing(8)
        } else {
            column![]
        };

        column![
            Text::new("Frequency Analysis").size(28),
            Text::new("Analyze character frequency in text. Supports Latin, Cyrillic, and other characters.").size(16),
            input_area,
            buttons_row,
            decrypt_row,
            Text::new(lang_text).size(14),
            error_text,
            Text::new("Frequency Table:").size(18),
            scrollable_table,
            decrypted_area,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    })
    .into()
}
