#[cfg(feature = "static_lookup")]
use std::{env, fs::File, io::Write, path::PathBuf};

#[cfg(feature = "static_lookup")]
const URL: &str =
    "https://raw.githubusercontent.com/deus-x-mackina/poker-lookup-table/main/codegen.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !cfg!(feature = "static_lookup") {
        return Ok(());
    }

    #[cfg(feature = "static_lookup")]
    {
        let path = env::var("OUT_DIR").map(PathBuf::from)?.join("codegen.rs");
        let mut file = File::create(path)?;
        file.write_all(&reqwest::blocking::get(URL)?.bytes()?)?;
    }

    Ok(())
}
