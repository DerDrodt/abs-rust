pub(crate) mod syntax;

use std::path::Path;

use xshell::{cmd, pushenv, write_file};

use anyhow::{bail, Result};

pub use syntax::generate_syntax;

const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/src/codegen`";

fn reformat(text: &str) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let stdout = cmd!("rustfmt --config fn_single_line=true")
        .stdin(text)
        .read()?;
    Ok(format!("//! {}\n\n{}\n", PREAMBLE, stdout))
}

fn ensure_rustfmt() -> Result<()> {
    let out = cmd!("rustfmt --version").read()?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}

fn update(path: &Path, contents: &str) -> Result<()> {
    eprintln!("updating {}", path.display());
    write_file(path, contents)?;
    return Ok(());
}
