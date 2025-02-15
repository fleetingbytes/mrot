use std::env::{self, VarError};

fn main() -> Result<(), VarError> {
    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let pkg_name_uppercase = pkg_name.to_uppercase();
    println!("cargo:rustc-env=PKG_NAME_UPPERCASE={}", pkg_name_uppercase);
    Ok(())
}
