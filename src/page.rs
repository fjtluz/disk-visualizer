use std::fmt::{Display, Formatter};

pub struct Line {
    position: String,
    hex_bytes: String,
    ascii_bytes: String,
}

impl Line {
    pub fn new() -> Line {
        Line {
            position: String::new(),
            hex_bytes: String::new(),
            ascii_bytes: String::new()
        }
    }

    pub fn create(position: String, hex_bytes: String, ascii_bytes: String) -> Line {
        Line {
            position,
            hex_bytes,
            ascii_bytes
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}\t{}\t{}", self.position, self.hex_bytes, self.ascii_bytes);
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        return Line {
            position: String::from(&self.position),
            hex_bytes: String::from(&self.hex_bytes),
            ascii_bytes: String::from(&self.hex_bytes)
        }
    }
}