use std::env;
use std::ffi::OsStr;
use std::process::Command;

fn rbconfig(key: &str) -> String {
    let ruby = match env::var_os("RUBY") {
        Some(val) => val.to_os_string(),
        None => OsStr::new("ruby").to_os_string(),
    };
    let config = Command::new(ruby)
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    String::from_utf8(config.stdout).expect("RbConfig value not UTF-8!")
}

fn use_libdir() {
    println!("cargo:rustc-link-search={}", rbconfig("libdir"));
}

fn transform_lib_args(rbconfig_key: &str, replacement: &str) -> String {
    rbconfig(rbconfig_key).replace("-l", replacement)
}

fn main() {
    use_libdir();
    println!("cargo:rustc-flags={}", transform_lib_args("LIBS", "-l "))
}
