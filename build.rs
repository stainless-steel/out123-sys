extern crate pkg_config;

use std::env;
use std::path::PathBuf;

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    if pkg_config::find_library("out123").is_ok() {
        return;
    }
    let dynamic = env::var("CARGO_FEATURE_STATIC").is_err();
    let output = PathBuf::from(get!("DEP_MPG123_ROOT"));
    println!(
        "cargo:rustc-link-lib={}=out123",
        if dynamic { "dylib" } else { "static" },
    );
    println!("cargo:rustc-link-search={}", output.join("lib").display());
}
