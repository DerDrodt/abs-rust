use std::{
    env,
    path::{Path, PathBuf},
};

pub mod codegen;

pub(crate) mod ast_src;

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
