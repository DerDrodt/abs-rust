use std::fmt;

pub struct Literal {
    pub s: String,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.s, f)
    }
}
