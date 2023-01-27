use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Port(8080)
    }
}

impl Port {
    pub fn as_u16(self) -> u16 {
        self.0
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Port(port)
    }
}
