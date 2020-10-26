use std::fmt;
pub struct CaseBranch<K> {
    pub pattern: Pattern,
    pub right: K,
}

impl<K> fmt::Display for CaseBranch<K>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.right)
    }
}

pub struct Pattern;

impl fmt::Display for Pattern {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
