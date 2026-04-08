//! `gfs version` — print the CLI version.

/// Print the current gfs CLI version.
pub fn run() {
    println!("gfs {}", env!("CARGO_PKG_VERSION"));
}
