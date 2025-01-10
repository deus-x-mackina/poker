#[cfg(feature = "static_lookup")]
use std::{env, fs::File, io::Write, path::PathBuf};

#[cfg(feature = "static_lookup")]
const URL: &str =
    "https://raw.githubusercontent.com/deus-x-mackina/poker-lookup-table/main/codegen.txt";

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
            Err(_) => panic!(
                "Unable to download static lookup tables with feature `static_lookup`. \
                Ensure you are connected to the internet and the build script has permissions to access \
                the internet."
            ),
            Ok(response) => response.bytes()?,
        };
        file.write_all(&bytes)?;
    }

    Ok(())
}
