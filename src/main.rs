mod page;

use page::Line;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::env;

use iced::{keyboard, Theme, window};
use iced::{subscription, Subscription, Event};
use iced::{Application, Error};

use iced::widget::{Button, container, horizontal_space, row, text, text_input};
use iced::widget::Column;
use iced::{Element, Length, Settings};
use iced::event::Status;
use iced::keyboard::KeyCode;


fn read_sector(path: &Path, start: u64) -> Vec<Line> {
    let mut utf_8_buffer = [0; 2];

    let mut page = vec![];

    match File::open(path) {
        Ok(mut file) => {

            if let Ok(offset) = file.seek(SeekFrom::Start(start))  {
                let mut hex_in_line = String::new();
                let mut str_in_line = String::new();

                let mut buffer = [0; 512];
                file.read_exact(&mut buffer).expect("Não foi possível ler o arquivo informado");

                let mut index = 0;
                for byte in buffer {
                    let byte_as_hex = format!("{:02X}", byte);
                    hex_in_line.push_str(byte_as_hex.as_str());
                    hex_in_line.push(' ');

                    match byte_as_hex.as_str() {
                        "00" => str_in_line.push('.'),
                        "85" => str_in_line.push_str("NL"),     // NEXT LINE
                        "0A" => str_in_line.push_str("LF"),     // LINE FEED
                        "0B" => str_in_line.push_str("LT"),     // LINE TABULATION
                        "0C" => str_in_line.push_str("FF"),     // FORM FEED
                        "0D" => str_in_line.push_str("CR"),     // CARRIAGE RETURN
                        _ => {
                            let byte_as_chr = char::from(byte);
                            let byte_as_str = byte_as_chr.encode_utf8(&mut utf_8_buffer);
                            str_in_line.push_str(byte_as_str);
                        }
                    }

                    index += 1;
                    if index % 16 == 0 {
                        let off_in_line = format!("{:05X}", offset + index);
                        page.push(Line::create(off_in_line, hex_in_line, str_in_line));

                        hex_in_line = String::new();
                        str_in_line = String::new();
                    }
                }
            }
            return page;
        },
        Err(_) => panic!("Não foi possível abrir o arquivo")
    }
}

fn find_term(path: &Path, search_term: &String) -> Option<(Vec<Line>, u64)> {
    let bytes = search_term.as_bytes();

    match File::open(path) {
        Ok(mut file) => {
            let mut offset = 0;

            loop {
                if let Ok(_) = file.seek(SeekFrom::Start(offset)) {
                    let mut buffer = [0; 10_000];

                    match file.read_exact(&mut buffer) {
                        Ok(..) => {
                            let mut contains_all_bytes = true;
                            for byte in bytes {
                                if !buffer.contains(byte) {
                                    contains_all_bytes = false;
                                    break;
                                }
                            }

                            if contains_all_bytes {
                                let mut matching_bytes = 0;
                                let mut index = 0;

                                for byte in buffer {
                                    if bytes[matching_bytes] == byte {
                                        matching_bytes += 1;
                                    } else {
                                        matching_bytes = 0;
                                    }

                                    if matching_bytes == bytes.len() {

                                        let mut word_location = offset + index - (bytes.len() as u64);
                                        while word_location % 16 != 0 {
                                            word_location -= 1;
                                        }

                                        return Some((read_sector(path, word_location), word_location));
                                    }
                                    index += 1;
                                }
                            }

                        }, Err(_) => break
                    }

                    offset += 10_000 - (bytes.len() as u64);
                } else {
                    return None;
                }
            }
            return None;
        },
        Err(_) => panic!("Não foi possível abrir o arquivo")
    }
}

enum Mode {
    READ,
    NAVE,
    FIND
}

fn load_font(setting: &mut Settings<()>) {
    let font_path = Path::new("fonts/JetBrainsMono-Regular.ttf");
    match File::open(font_path) {
        Ok(mut file) => {
            let mut buffer = vec![0; 273_900];
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    let boxed_buffer: Box<[u8]> = buffer.into_boxed_slice();
                    let font: &'static [u8] = Box::leak(boxed_buffer);
                    setting.default_font = Some(font);
                }
                Err(_) => println!("Não foi possível definir a fonte"),
            }
        }
        Err(_) => println!("Não foi possível definir a fonte"),
    }
}

fn main() -> Result<(), Error> {
    let mut settings = Settings {
        window: window::Settings {
            size: (750, 660),
            resizable: false,
            ..window::Settings::default()
        },
        ..Default::default()
    };

    load_font(&mut settings);

    DiskVisualizer::run(settings)
}

pub struct DiskVisualizer<'a> {
    path: &'a Path,
    current_page: Vec<Line>,
    start: u64,
    string_input: String,
    operation_mode: Mode,
}

impl DiskVisualizer<'_> {
    pub fn load_page(&mut self, start: u64) {
        self.current_page = read_sector(self.path, start);
        self.start = start;
    }

    pub fn find_term(&mut self) -> Option<(Vec<Line>, u64)> {
        return find_term(self.path, &self.string_input);
    }
}

impl Application for DiskVisualizer<'_> {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let disk_path = match env::consts::OS {
            "windows" => Path::new("\\\\.\\C:"),
            "linux" => Path::new("/dev/sda"),
            _ => panic!("Sistema operacional não suportado")
        };

        let page = read_sector(disk_path, 0);

        (
            DiskVisualizer {
                path: disk_path,
                current_page: page,
                start: 0,
                string_input: String::new(),
                operation_mode: Mode::READ,
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("disk_visualizer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::PageDown => {
                self.load_page(self.start + 512);
            },
            Message::PageUp => {
                if self.start >= 30 {
                    self.load_page(self.start - 512);
                } else if self.start != 0 {
                    self.load_page(0);
                }
            },
            Message::Down => {
                self.load_page(self.start + 16);
            },
            Message::Up => {
                if self.start >= 1 {
                    self.load_page(self.start - 16);
                }
            },
            Message::Esc => {
                self.operation_mode = Mode::READ;
                self.string_input = String::new();
            }
            Message::Find => self.operation_mode = Mode::FIND,
            Message::Navigate => self.operation_mode = Mode::NAVE,
            Message::InputChange(str) => self.string_input = str,
            Message::SubmitInput => {
                if let Mode::FIND = self.operation_mode {
                    if let Some(page) = self.find_term() {
                        self.current_page = page.0;
                        self.start = page.1;
                    }
                } else if let Mode::NAVE = self.operation_mode {
                    match u64::from_str_radix(self.string_input.as_str(), 16) {
                        Ok(hash) => {
                            self.current_page = read_sector(self.path, hash);
                            self.start = hash;
                        }
                        Err(_) => println!("Não possível converter")
                    }
                }
            }
        }
        return iced::Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let navigate_btn = Button::new(text("Navigate").size(12))
            .on_press(Message::Navigate);

        let find_btn = Button::new(text("Find").size(12))
            .on_press(Message::Find);

        let input_text = text_input("", self.string_input.as_str())
            .on_input(Message::InputChange)
            .on_submit(Message::SubmitInput)
            .width(300)
            .size(12);

        let horizontal_space = horizontal_space(20);

        let mut mode_display = match self.operation_mode {
            Mode::READ => text("READ MODE"),
            Mode::FIND => text("FIND MODE"),
            Mode::NAVE => text("NAVE MODE")
        };

        mode_display = mode_display.size(24);

        let mut toolbar = row![navigate_btn, find_btn, mode_display, horizontal_space];
        toolbar = toolbar.spacing(5);

        if let Mode::FIND | Mode::NAVE = self.operation_mode {
            toolbar = toolbar.push(input_text);
        }

        let mut content: Column<'_, Message> = Column::new().width(Length::Fill);
        content = content.push(toolbar);

        for line in &self.current_page {
            content = content.push(
                text(format!("{}", line))
            );
        }

        container(content).center_x().into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::PageDown,
                    modifiers: _,
                    ..
                }),
                Status::Ignored
            ) => Some(Message::PageDown),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::PageUp,
                    modifiers: _,
                    ..
                }),
                Status::Ignored
            ) => Some(Message::PageUp),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key_code: KeyCode::Down,
                                    modifiers: _,
                                    ..
                                }),
                Status::Ignored
            ) => Some(Message::Down),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key_code: KeyCode::Up,
                                    modifiers: _,
                                    ..
                                }),
                Status::Ignored
            ) => Some(Message::Up),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                                    key_code: KeyCode::Escape,
                                    modifiers: _,
                                    ..
                                }),
                Status::Ignored
            ) => Some(Message::Esc),
            _ => None
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PageDown,
    PageUp,
    Down,
    Up,
    Esc,
    Navigate,
    Find,
    InputChange(String),
    SubmitInput,
}