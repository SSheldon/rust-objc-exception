extern crate gcc;

use std::env;

fn main() {
    let mut config = gcc::Config::new();

    // It seems that clang will try to compile for OSX even if you ask it to
    // compile for i386-apple-ios; this manifests as the compiler generating
    // code that links against symbols like objc_exception_extract that are
    // only available on OSX, not iOS.
    // The only way to avoid this seems to be to set the OS version min.
    let target = env::var("TARGET").unwrap();
    let version_min_flag = if target.contains("-ios") {
        let version = env::var("IPHONEOS_DEPLOYMENT_TARGET").unwrap_or("2.0".to_string());
        let platform = if target.contains("i386") ||
                target.contains("i686") ||
                target.contains("x86_64") {
            "ios-simulator"
        } else {
            "iphoneos"
        };
        Some((platform, version))
    } else if target.contains("-darwin") {
        let version = env::var("MACOSX_DEPLOYMENT_TARGET").unwrap_or("10.4".to_string());
        Some(("macosx", version))
    } else {
        None
    };

    if let Some((platform, version)) = version_min_flag {
        config.flag(&format!("-m{}-version-min={}", platform, version));
    }

    config.file("extern/exception.m");
    config.compile("libexception.a");
}
