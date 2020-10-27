use anyhow::Result;

fn main() -> Result<()> {
    xtask::codegen::generate_syntax()
}
