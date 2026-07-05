use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Override rodata.x to place .flash.appdesc at the START of .rodata
    // Our OUT_DIR comes first in the linker search path, so this
    // shadows the original rodata.x from esp-hal.
    // The ESP-IDF bootloader expects the app descriptor at offset 0
    // of the page-aligned DROM segment (0x3c010020).
    fs::write(
        out_dir.join("rodata.x"),
        r#"
SECTIONS {
  .rodata : ALIGN(4)
  {
    . = ALIGN (4);
    _rodata_start = ABSOLUTE(.);
    KEEP(*(.flash.appdesc))
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
    . = ALIGN(4);
    _rodata_end = ABSOLUTE(.);
  } > RODATA

  .rodata.wifi : ALIGN(4)
  {
    . = ALIGN(4);
    *( .rodata_wlog_*.* )
    . = ALIGN(4);
  } > RODATA
}
"#,
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=build.rs");
}
