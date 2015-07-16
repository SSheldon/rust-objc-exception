extern crate gcc;

use std::env;

fn main() {
    let mut config = gcc::Config::new();

    if let Ok(ios_version) = env::var("IPHONEOS_DEPLOYMENT_TARGET") {
        let platform = env::var("PLATFORM_NAME");
        if platform.map(|p| p == "iphonesimulator").unwrap_or(false) {
            config.flag(&format!("-mios-simulator-version-min={}", ios_version));
        } else {
            config.flag(&format!("-miphoneos-version-min={}", ios_version));
        }
    }

    if let Ok(osx_version) = env::var("MACOSX_DEPLOYMENT_TARGET") {
        config.flag(&format!("-mmacosx-version-min={}", osx_version));
    }

    config.file("extern/exception.m");
    config.compile("libexception.a");
}
