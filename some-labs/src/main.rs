mod ciphers;
mod cipher_ui;

use iced::Task;
use iced::Theme;
use iced::{Element, Length, widget::{column, container, responsive}};

use ciphers::{atbash_cipher, caesar_cipher, gronsfeld_cipher, parse_rishelau_mask, rishelau_cipher, vigenere_cipher, frequency_analysis, FrequencySubstitution, Language, gamming_encrypt, gamming_decrypt, hex_encode, hex_decode};
use cipher_ui::{draw_atbash, draw_ceasar, draw_gronsfeld, draw_rishelau, draw_cipher_selector, draw_vigenere, draw_frequency_analysis, draw_gamming};

#[derive(Debug, Clone)]
pub enum Message {
    Atbash,
    Ceasar,
    Rishelau,
    Gronsfeld,
    Vigenere,
    Gamming,
    FrequencyAnalysis,
    AtbashInput(String),
    AtbashOutputIgnored(String),
    CeasarInput(String),
    CeasarShift(String),
    CeasarOutputIgnored(String),
    RishelauInput(String),
    RishelauOutput(String),
    RishelauMask(String),
    GronsfeldInput(String),
    GronsfeldKey(String),
    GronsfeldOutputIgnored(String),
    VigenereInput(String),
    VigenereKey(String),
    VigenereOutputIgnored(String),
    FreqAnalysisInput(String),
    FreqAnalysisPaste,
    FreqAnalysisLoadFile,
    FreqAnalysisFileLoaded(String, String),
    FreqAnalysisDecrypt(Language),
    FreqAnalysisDecrypted(String),
    GammingInput(String),
    GammingSeed(String),
    GammingShiftA(String),
    GammingShiftB(String),
    GammingShiftC(String),
    GammingOutputIgnored(String),
    GammingEncrypt,
    GammingDecrypt,
    FreqAnalysisSwapA(String),
    FreqAnalysisSwapB(String),
    FreqAnalysisSwap,
}

#[derive(Default)]
enum RishelauLastEdited {
    #[default]
    Input,
    Output,
}

// rough transcriptions from Russian
#[derive(Default)]
pub enum Ciphers {
    #[default]
    ATBASH,
    CEASAR,
    RISHELAU,
    GRONSFELD,
    VIGENERE,
    GAMMING,
    FrequencyAnalysis,
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
    gronsfeld_input: String,
    gronsfeld_key: String,
    gronsfeld_output: String,
    vigenere_input: String,
    vigenere_key: String,
    vigenere_output: String,
    gamming_input: String,
    gamming_seed: String,
    gamming_shift_a: String,
    gamming_shift_b: String,
    gamming_shift_c: String,
    gamming_output: String,
    gamming_decrypted: String,
    freq_analysis_input: String,
    freq_analysis_result: Vec<(char, usize, f64)>,
    freq_analysis_error: String,
    freq_analysis_decrypted: String,
    freq_analysis_selected_lang: Language,
    freq_substitution: Option<FrequencySubstitution>,
    freq_swap_input_a: String,
    freq_swap_input_b: String,
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
                draw_cipher_selector(),
                match self.cipher_selected {
                    Ciphers::ATBASH => draw_atbash(&self.atbash_input, &self.atbash_output),
                    Ciphers::CEASAR => draw_ceasar(&self.ceasar_input, &self.ceasar_shift, &self.ceasar_output),
                    Ciphers::RISHELAU => draw_rishelau(&self.rishelau_input, &self.rishelau_output, &self.rishelau_mask, &self.rishelau_error),
                    Ciphers::GRONSFELD => draw_gronsfeld(&self.gronsfeld_input, &self.gronsfeld_key, &self.gronsfeld_output),
                    Ciphers::VIGENERE => draw_vigenere(&self.vigenere_input, &self.vigenere_key, &self.vigenere_output),
                    Ciphers::GAMMING => draw_gamming(&self.gamming_input, &self.gamming_seed, &self.gamming_shift_a, &self.gamming_shift_b, &self.gamming_shift_c, &self.gamming_output, &self.gamming_decrypted),
                    Ciphers::FrequencyAnalysis => draw_frequency_analysis(&self.freq_analysis_input, &self.freq_analysis_result, &self.freq_analysis_error, &self.freq_analysis_decrypted, self.freq_analysis_selected_lang, &self.freq_substitution, &self.freq_swap_input_a, &self.freq_swap_input_b),
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Atbash => {
                self.cipher_selected = Ciphers::ATBASH;
                Task::none()
            }
            Message::Ceasar => {
                self.cipher_selected = Ciphers::CEASAR;
                Task::none()
            }
            Message::Rishelau => {
                self.cipher_selected = Ciphers::RISHELAU;
                Task::none()
            }
            Message::AtbashInput(input) => {
                self.atbash_input = input;
                self.atbash_output = atbash_cipher(&self.atbash_input);
                Task::none()
            }
            Message::AtbashOutputIgnored(_input) => Task::none(),
            Message::CeasarInput(input) => {
                self.ceasar_input = input;
                let shift = self.ceasar_shift.trim().parse::<i32>().unwrap_or(0);
                self.ceasar_output = caesar_cipher(&self.ceasar_input, shift);
                Task::none()
            }
            Message::CeasarShift(shift) => {
                self.ceasar_shift = shift;
                let shift = self.ceasar_shift.trim().parse::<i32>().unwrap_or(0);
                self.ceasar_output = caesar_cipher(&self.ceasar_input, shift);
                Task::none()
            }
            Message::CeasarOutputIgnored(_input) => Task::none(),
            Message::RishelauInput(input) => {
                self.rishelau_input = input;
                self.rishelau_last_edited = RishelauLastEdited::Input;
                self.recalc_rishelau();
                Task::none()
            }
            Message::RishelauOutput(output) => {
                self.rishelau_output = output;
                self.rishelau_last_edited = RishelauLastEdited::Output;
                self.recalc_rishelau();
                Task::none()
            }
            Message::RishelauMask(mask) => {
                self.rishelau_mask = mask;
                self.recalc_rishelau();
                Task::none()
            }
            Message::Gronsfeld => {
                self.cipher_selected = Ciphers::GRONSFELD;
                Task::none()
            }
            Message::GronsfeldInput(input) => {
                self.gronsfeld_input = input;
                self.gronsfeld_output = gronsfeld_cipher(&self.gronsfeld_input, &self.gronsfeld_key, false);
                Task::none()
            }
            Message::GronsfeldKey(key) => {
                self.gronsfeld_key = key;
                self.gronsfeld_output = gronsfeld_cipher(&self.gronsfeld_input, &self.gronsfeld_key, false);
                Task::none()
            }
            Message::GronsfeldOutputIgnored(_input) => Task::none(),
            Message::Vigenere => {
                self.cipher_selected = Ciphers::VIGENERE;
                Task::none()
            }
            Message::VigenereInput(input) => {
                self.vigenere_input = input;
                self.vigenere_output = vigenere_cipher(&self.vigenere_input, &self.vigenere_key, false);
                Task::none()
            }
            Message::VigenereKey(key) => {
                self.vigenere_key = key;
                self.vigenere_output = vigenere_cipher(&self.vigenere_input, &self.vigenere_key, false);
                Task::none()
            }
            Message::VigenereOutputIgnored(_input) => Task::none(),
            Message::Gamming => {
                self.cipher_selected = Ciphers::GAMMING;
                Task::none()
            }
            Message::GammingInput(input) => {
                self.gamming_input = input;
                self.gamming_output.clear();
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingSeed(seed) => {
                self.gamming_seed = seed;
                self.gamming_output.clear();
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingShiftA(val) => {
                self.gamming_shift_a = val;
                self.gamming_output.clear();
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingShiftB(val) => {
                self.gamming_shift_b = val;
                self.gamming_output.clear();
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingShiftC(val) => {
                self.gamming_shift_c = val;
                self.gamming_output.clear();
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingOutputIgnored(_input) => Task::none(),
            Message::GammingEncrypt => {
                let seed: u32 = self.gamming_seed.trim().parse().unwrap_or(1);
                let a: u32 = self.gamming_shift_a.trim().parse().unwrap_or(13);
                let b: u32 = self.gamming_shift_b.trim().parse().unwrap_or(17);
                let c: u32 = self.gamming_shift_c.trim().parse().unwrap_or(5);
                let ciphertext = gamming_encrypt(&self.gamming_input, seed, a, b, c);
                self.gamming_output = hex_encode(&ciphertext);
                self.gamming_decrypted.clear();
                Task::none()
            }
            Message::GammingDecrypt => {
                let seed: u32 = self.gamming_seed.trim().parse().unwrap_or(1);
                let a: u32 = self.gamming_shift_a.trim().parse().unwrap_or(13);
                let b: u32 = self.gamming_shift_b.trim().parse().unwrap_or(17);
                let c: u32 = self.gamming_shift_c.trim().parse().unwrap_or(5);
                match hex_decode(&self.gamming_input) {
                    Ok(bytes) => {
                        self.gamming_decrypted = gamming_decrypt(&bytes, seed, a, b, c);
                        self.gamming_output.clear();
                    }
                    Err(e) => {
                        self.gamming_output = format!("Hex decode error: {}", e);
                        self.gamming_decrypted.clear();
                    }
                }
                Task::none()
            }
            Message::FrequencyAnalysis => {
                self.cipher_selected = Ciphers::FrequencyAnalysis;
                Task::none()
            }
            Message::FreqAnalysisInput(input) => {
                self.freq_analysis_input = input;
                self.freq_analysis_result = frequency_analysis(&self.freq_analysis_input);
                self.freq_analysis_error.clear();
                self.freq_analysis_decrypted.clear();
                self.freq_substitution = None;
                self.freq_swap_input_a.clear();
                self.freq_swap_input_b.clear();
                Task::none()
            }
            Message::FreqAnalysisPaste => {
                // В реальном приложении здесь был бы доступ к буферу обмена
                // Пока просто очищаем ошибку
                self.freq_analysis_error = "Paste functionality requires clipboard access".to_string();
                Task::none()
            }
            Message::FreqAnalysisLoadFile => {
                // В реальном приложении здесь был бы диалог выбора файла
                // Пока просто очищаем ошибку
                self.freq_analysis_error = "File loading requires async file dialog".to_string();
                Task::none()
            }
            Message::FreqAnalysisFileLoaded(content, filename) => {
                self.freq_analysis_input = content;
                self.freq_analysis_result = frequency_analysis(&self.freq_analysis_input);
                self.freq_analysis_error = format!("Loaded file: {}", filename);
                self.freq_analysis_decrypted.clear();
                Task::none()
            }
            Message::FreqAnalysisDecrypt(lang) => {
                self.freq_analysis_selected_lang = lang;
                let sub = FrequencySubstitution::new(&self.freq_analysis_input, lang);
                self.freq_analysis_decrypted = sub.decrypt(&self.freq_analysis_input);
                self.freq_substitution = Some(sub);
                self.freq_swap_input_a.clear();
                self.freq_swap_input_b.clear();
                Task::none()
            }
            Message::FreqAnalysisDecrypted(_result) => Task::none(),
            Message::FreqAnalysisSwapA(input) => {
                self.freq_swap_input_a = input.chars().last().map(|c| c.to_string()).unwrap_or_default();
                Task::none()
            }
            Message::FreqAnalysisSwapB(input) => {
                self.freq_swap_input_b = input.chars().last().map(|c| c.to_string()).unwrap_or_default();
                Task::none()
            }
            Message::FreqAnalysisSwap => {
                if let Some(ref mut substitution) = self.freq_substitution {
                    let a = self.freq_swap_input_a.chars().next();
                    let b = self.freq_swap_input_b.chars().next();
                    if let (Some(a_char), Some(b_char)) = (a, b) {
                        substitution.swap(a_char, b_char);
                        self.freq_analysis_decrypted = substitution.decrypt(&self.freq_analysis_input);
                    }
                }
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(Theme::Light)
        .window_size((900, 500))
        .resizable(true)
        .run()
}
