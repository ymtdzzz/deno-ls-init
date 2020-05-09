extern crate deno_ls_init;

use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;
use std::io::{Write};

use deno_ls_init::*;

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

    std::process::Command::new("npm")
        .args(&["install", "--save-dev", "typescript-deno-plugin", "typescript"])
        .output()
        .expect("npm install failed.");

    Ok(())
}

