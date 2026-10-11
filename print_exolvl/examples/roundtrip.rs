//! Round-trip check: read each level, write it back and compare the bytes.
//!
//! Run it from the project folder:
//!   cargo run --manifest-path print_exolvl\Cargo.toml --example roundtrip -- samples
//!   cargo run --manifest-path print_exolvl\Cargo.toml --example roundtrip -- samples\allactions.exolvl
//!
//! The argument may be a single .exolvl file or a folder (default: samples).
//! The comparison is made on the decompressed bytes, so gzip settings do not matter.

use exolvl::{Read, Write, gzip::extract, types::exolvl::Exolvl};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn check(path: &Path) -> bool {
    let name = path.display();

    let raw = match std::fs::read(path) {
        Ok(raw) => raw,
        Err(e) => {
            println!("FAIL  {name}: cannot read file: {e}");
            return false;
        }
    };

    let in_bytes = match extract(&raw) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("FAIL  {name}: cannot decompress: {e:?}");
            return false;
        }
    };

    let level = match Exolvl::read(&mut Cursor::new(in_bytes.as_slice())) {
        Ok(level) => level,
        Err(e) => {
            println!("FAIL  {name}: read error: {e:?}");
            return false;
        }
    };

    let mut out_bytes = Vec::new();
    if let Err(e) = level.write(&mut out_bytes) {
        println!("FAIL  {name}: write error: {e:?}");
        return false;
    }

    if in_bytes == out_bytes {
        println!("OK    {name} ({} bytes)", in_bytes.len());
        return true;
    }

    println!(
        "DIFF  {name}: original {} bytes, written {} bytes",
        in_bytes.len(),
        out_bytes.len()
    );

    match in_bytes.iter().zip(out_bytes.iter()).position(|(a, b)| a != b) {
        Some(at) => {
            let from = at.saturating_sub(8);
            let to_in = (at + 24).min(in_bytes.len());
            let to_out = (at + 24).min(out_bytes.len());
            println!("      first difference at byte {at} (starting 8 bytes earlier)");
            println!("      original: {}", hex(&in_bytes[from..to_in]));
            println!("      written : {}", hex(&out_bytes[from..to_out]));
        }
        None => {
            println!("      one is a prefix of the other (lengths differ only)");
        }
    }

    false
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "samples".to_string());
    let target = PathBuf::from(arg);

    let mut files: Vec<PathBuf> = Vec::new();
    if target.is_dir() {
        let entries = std::fs::read_dir(&target).expect("cannot read folder");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "exolvl") {
                files.push(path);
            }
        }
        files.sort();
    } else {
        files.push(target);
    }

    let total = files.len();
    let passed = files.iter().filter(|f| check(f)).count();

    println!();
    println!("{passed} of {total} files round-tripped byte for byte.");
}
