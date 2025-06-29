use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("wat2wasm")
        .args(["contracts/bitcode20/token.wat", "-o", "contracts/bitcode20/token.wasm"])
        .status()
        .expect("Failed to compile WAT to WASM");
}

fn main() {
    // Set output directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // === Optional: Compile or copy WASM contracts ===
    let wasm_contracts = vec![
        "contracts/bitcode20/token.wasm",
    ];

    for contract in wasm_contracts {
        let target_path = Path::new(&out_dir).join(Path::new(contract).file_name().unwrap());
        println!("cargo:rerun-if-changed={}", contract);

        // Copy the .wasm contract into the build output directory
        fs::copy(contract, &target_path).expect("Failed to copy WASM contract");
    }

    // === Optional: Embed static web UI or config ===
    let static_files = vec!["static/index.html", "static/style.css"];
    for file in static_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    // You could include embedded HTML or templates here, too.
}
