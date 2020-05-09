extern crate denoinit;

use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;
use std::io::{Write};

use denoinit::*;

fn main() -> Result<()> {
    let config_info = ConfigInfo::new().with_context(|| "Failed to get config info")?;
    let config_path = "tsconfig.json";
    let mut json_str = "".to_string();

    if Path::new(&config_path).exists() {
        json_str = std::fs::read_to_string(&config_path)?;
    }

    let tsconfig = deno_init(json_str, &config_info);

    let mut f = File::create("tsconfig.json")?;
    write!(f, "{}", tsconfig?);

    Ok(())
}

