#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [package]
//! edition = "2024"
//! ```

use std::env::args;
use std::fs::{copy, create_dir_all};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::process::Command;

const CODE: &str = "./tools/ovmf/x86_64/code.fd";
const VARS: &str = "./tools/ovmf/x86_64/vars.fd";

fn main() -> Result<()> {
    let target = args().nth(1).ok_or(Error::new(
        ErrorKind::InvalidInput,
        "Missing target argument",
    ))?;

    let binary = args().nth(2).ok_or(Error::new(
        ErrorKind::InvalidInput,
        "Missing binary argument",
    ))?;

    let esp = Path::new(&binary).parent().unwrap().join("esp");
    let boot = esp.join("efi").join("boot");
    let dst = esp.join("bootx64.efi");

    create_dir_all(&boot)?;
    copy(&binary, &dst)?;

    let qemu = match target.as_str() {
        "aarch64-unknown-uefi" => "qemu-system-aarch64",
        "x86_64-unknown-uefi" => "qemu-system-x86_64",
        _ => unimplemented!(),
    };

    Command::new(qemu)
        .args(&[
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", CODE),
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", VARS),
            "-drive",
            &format!("format=raw,file=fat:rw:{}", esp.display()),
        ])
        .status()?;

    Ok(())
}
