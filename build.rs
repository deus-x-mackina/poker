#[cfg(feature = "static_lookup")]
use std::{env, fs::File, io::Write, path::PathBuf};

#[cfg(feature = "static_lookup")]
const URL: &str = "https://raw.githubusercontent.com/deus-x-mackina/poker-lookup-table/ab48033c149d524098c1f921f1ba9989fd4c2b03/codegen.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !cfg!(feature = "static_lookup") {
        return Ok(());
    }

    let docsrs = std::env::var("DOCS_RS").unwrap_or_default();
    if docsrs == "1" {
        return Ok(());
    }

    #[cfg(feature = "static_lookup")]
    {
        let path = env::var("OUT_DIR").map(PathBuf::from)?.join("codegen.rs");
        let mut file = File::create(path)?;
        let bytes = match reqwest::blocking::get(URL) {
            Err(e) => panic!(
                "Failed to download static lookup tables for `static_lookup` feature.\nError: \
                 {}\nSolutions:\n- Ensure you have an active internet connection\n- Check if your \
                 firewall/proxy allows access to GitHub\n- Try building without the \
                 `static_lookup` feature: cargo build --no-default-features\n- If offline, \
                 consider using the default runtime table generation instead",
                e
            ),
            Ok(response) => response.bytes()?,
        };
        file.write_all(&bytes)?;
    }

    Ok(())
}
