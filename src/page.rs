use std::fmt::{Display, Formatter};

pub struct Line {
    position: String,
    hex_bytes: String,
    pub ascii_bytes: String,
}

impl Line {
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
        return write!(f, "{}      {}      {}", self.position, self.hex_bytes, self.ascii_bytes);
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        return Line {
            position: String::from(&self.position),
            hex_bytes: String::from(&self.hex_bytes),
            ascii_bytes: String::from(&self.ascii_bytes)
        }
    }
}