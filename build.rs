extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.ends_with("apple-darwin") || target.ends_with("apple-ios") {
        gcc::compile_library("libexception.a", &["extern/exception.m"]);
    }
}
