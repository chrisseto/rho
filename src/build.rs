extern crate pkg_config;

fn main() {
    println!("cargo:rustc-link-search=/usr/local/opt/ncurses/lib");
}
