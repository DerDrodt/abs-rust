use ungrammar::Grammar;

pub fn abs_grammar() -> Grammar {
    let src = include_str!("./abs.ungram");
    src.parse().unwrap()
}

pub fn generate_syntax() {}
