extern crate core;

mod page;

use page::Line;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::env;

use iced::{keyboard, window};
use iced::{subscription, Subscription, Event};
use iced::{Application, Error};
use iced::theme;

use iced::widget::{container,  text};
use iced::widget::Column;
use iced::{Element, Length, Settings};
use iced::event::Status;
use iced::keyboard::KeyCode;

fn write_to_page(file: &mut File, page: &mut Vec<Line>, start: usize, end: usize) {
    let mut total_of_bytes_read = start.clone();
    let mut index = 0;

    let mut hex_in_line = String::new();
    let mut ascii_in_line = String::new();

    let reader = BufReader::new(file);

    let bytes = reader.bytes();

    for byte_result in bytes.skip(10 * start) {
        match byte_result {
            Ok(byte) => {
                let mut byte_as_hex = format!("{byte:X}");
                if byte_as_hex.len() == 1 {
                    byte_as_hex.insert(0, '0');
                }
                hex_in_line.push_str(byte_as_hex.as_str());
                hex_in_line.push(' ');

                let byte_as_ascii = char::from(byte);

                match byte_as_hex.as_str() {
                    "85" => ascii_in_line.push_str("NL"),   // NEXT LINE (NL)
                    "0A" => ascii_in_line.push_str("LF"),   // LINE FEED (LF)
                    "0B" => ascii_in_line.push_str("LT"),   // LINE TABULATION (LT)
                    "0C" => ascii_in_line.push_str("FF"),   // FORM FEED (FF)
                    "0D" => ascii_in_line.push_str("CR"),   // CARRIAGE RETURN (CR)
                    _ => ascii_in_line.push(byte_as_ascii)
                }
            },
            Err(e) => println!("{}", e)
        };

        index += 1;
        if index % 10 == 0 {
            let line = format!("{:08}", index + (10 * start));
            page.push(Line::create(line, hex_in_line, ascii_in_line));

            hex_in_line = String::new();
            ascii_in_line = String::new();

            total_of_bytes_read += 1;
        }

        if total_of_bytes_read == end {
            break;
        }
    }
}

fn main() -> Result<(), Error> {
    let settings = Settings {
        window: window::Settings {
            size: (600, 600),
            resizable: false,
            ..window::Settings::default()
        },
        ..Default::default()
    };

    DiskVisualizer::run(settings)
}

pub struct DiskVisualizer {
    file: File,
    current_page: Vec<Line>,
    start: usize,
    end: usize
}

impl DiskVisualizer {
    pub fn load_page(&mut self, start: usize, end: usize) {
        let mut current_page = vec![];

        write_to_page(&mut self.file, &mut current_page, start, end);

        self.current_page = current_page;
        self.start = start;
        self.end = end;
    }

    pub fn print_page(&self) {
        for line in &self.current_page {
            println!("{} {}", line.position, line.hex_bytes);
        }
    }
}

impl Application for DiskVisualizer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let mut disk_path = Path::new("");
        if env::consts::OS == "linux" {
            disk_path = Path::new("/dev/sda");
        } else if env::consts::OS == "windows" {
            disk_path = Path::new("\\\\.\\C:");
        } else {
            panic!("OS não suportado pela aplicação!")
        }

        let mut file = File::open(disk_path).expect("Não foi possível ler o arquivo informado!");

        let mut page: Vec<Line> = vec![];

        write_to_page(&mut file, &mut page, 0, 30);

        (
            DiskVisualizer {
                file,
                current_page: page,
                start: 0,
                end: 30
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
                self.load_page(self.start + 30, self.end + 30);
            },
            Message::PageUp => {
                if self.start >= 30 {
                    self.load_page(self.start - 30, self.end - 30);
                } else if self.start != 0 {
                    self.load_page(0, 30);
                }
            },
            Message::Down => {
                self.load_page(self.start + 1, self.end + 1);
            },
            Message::Up => {
                if self.start >= 1 {
                    self.load_page(self.start - 1, self.end - 1);
                }
            }
        }
        return iced::Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.print_page();

        let mut content: Column<'_, Message> = Column::new().width(Length::Fill);

        for i in &self.current_page {
            content = content.push(
                text(format!("{i}"))
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
            _ => None
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PageDown,
    PageUp,
    Down,
    Up
}



