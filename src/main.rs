mod page;

use page::Line;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use iced::{alignment, Error};
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row,
    scrollable, slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Color, Element, Font, Length, Renderer, Sandbox, Settings};

fn write_to_page(file: &mut File, page: &mut Vec<Line>, offset: usize) {
    let mut total_of_bytes_read = 0;
    let mut index = 0;

    let mut hex_in_line = String::new();
    let mut ascii_in_line = String::new();

    let reader = BufReader::new(file);

    let bytes = reader.bytes();

    for byte_result in bytes.skip(1000 * offset) {
        match byte_result {
            Ok(byte) => {
                let mut byte_as_hex = format!("{byte:X} ");
                if byte_as_hex.len() == 2 {
                    byte_as_hex.insert(0, '0');
                }
                hex_in_line.push_str(byte_as_hex.as_str());

                let mut byte_as_ascii = char::from(byte).to_string();
                if byte_as_ascii == "\n" {
                    byte_as_ascii = String::from("\\n");
                }
                ascii_in_line.push_str(byte_as_ascii.as_str());
            },
            Err(e) => println!("{}", e)
        };

        index += 1;
        if index % 10 == 0 {
            let line = format!("{:08}", index + (1000 * offset));
            page.push(Line::create(line, hex_in_line, ascii_in_line));

            hex_in_line = String::new();
            ascii_in_line = String::new();
        }

        total_of_bytes_read += 1;
        if total_of_bytes_read == 1000 {
            break;
        }
    }
}

fn main() -> Result<(), Error> {
    DiskVisualizer::run(Settings::default())
}

pub struct DiskVisualizer {
    file: File,
    indexes: [u8; 3],
    pages: [Vec<Line>; 3],
    current_page: Vec<Line>,
    debug: bool,
}

impl Sandbox for DiskVisualizer {
    type Message = Message;

    fn new() -> Self {
        let disk_path = Path::new("/dev/sda");
        let mut file = File::open(disk_path).expect("Não foi possível ler o arquivo informado!");

        let mut pages: [Vec<Line>; 3] = [vec![], vec![], vec![]];

        for i in 0..=2 {
            write_to_page(&mut file, &mut pages[i], i);
        }

        let mut current_page = vec![];
        for line in &pages[0] {
            current_page.push(line.clone())
        }

        DiskVisualizer {
            file,
            indexes: [0, 1, 2],
            pages,
            current_page,
            debug: false
        }
    }

    fn title(&self) -> String {
        String::from("disk_visualizer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PageUp => {
                println!("page up!");
            },
            Message::PageDown => {
                println!("page down");
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut content: Column<'_, Message> = column![];

        for i in &self.current_page {
            content = content.push(text(format!("{i}")));
        }

        container(content).height(Length::Fill).width(Length::Fill).center_x().into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PageUp,
    PageDown
}

