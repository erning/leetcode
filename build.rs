use std::fs;
use std::io::prelude::*;
use std::path::Path;

extern crate glob;
use crate::glob::glob;

fn main() -> std::io::Result<()> {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = ".";
    let dest_path = Path::new(&out_dir).join("lib.rs");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_path)?;

    for e in glob("*/init.rs").expect("Failed to read glob pattern") {
        let pb = e.unwrap();
        let item = pb.to_str().unwrap();
        writeln!(file, "#[path=\"{}\"]", item)?;
        writeln!(file, "pub mod problem_{};", item.get(0..4).unwrap())?;
        writeln!(file)?;
    }

    Ok(())
}
