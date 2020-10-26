use std::fmt;

pub struct Ident {
    pub str: String,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.str, f)
    }
}
