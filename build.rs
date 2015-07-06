extern crate gcc;

use std::env;

fn main() {
    let mut config = gcc::Config::new();

    let target = env::var("TARGET").unwrap();
    if target.contains("ios") {
        if target.contains("i386") ||
                target.contains("i686") ||
                target.contains("x86_64") {
            // Simulator
            config.flag("-mios-simulator-version-min=4.2");
        } else {
            // Device
            config.flag("-miphoneos-version-min=4.2");
        }
    } else if target.contains("darwin") {
        // OSX
        config.flag("-mmacosx-version-min=10.3");
    }

    config.file("extern/exception.m");
    config.compile("libexception.a");
}
