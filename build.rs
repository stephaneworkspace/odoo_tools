extern crate cbindgen;

use cbindgen::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let _output_file = target_dir()
        .join(format!("{}.h", package_name))
        .display()
        .to_string();
    let mut config: Config = Default::default();
    config.language = cbindgen::Language::C;
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(out_path.join("libodoo.h"));
    println!("{:?}", out_path.join("libodoo.h"));
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
