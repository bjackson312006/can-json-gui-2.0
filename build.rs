fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");

    if target_os.as_deref() == Ok("windows") {
        let _ = embed_resource::compile("packaging/icon.rc", embed_resource::NONE);
    }

    if target_os.as_deref() == Ok("linux") {
        warn_on_missing_linux_deps();
    }
}

/// Warn people when they try to build on linux with missing deps.
fn warn_on_missing_linux_deps() {
    // (pkg-config name, dev package hint)
    const REQUIRED: &[&str] = &[
        "xkbcommon",
        "xkbcommon-x11",
        "xcb",
        "wayland-client",
        "fontconfig",
        "freetype2",
        "openssl",
    ];

    // If pkg-config itself is missing we can't check anything reliably.
    let pkg_config_available = std::process::Command::new("pkg-config")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !pkg_config_available {
        println!(
            "cargo:warning=pkg-config not found; cannot verify native dependencies. \
             Run ./scripts/setup.sh to install them."
        );
        return;
    }

    let missing: Vec<&str> = REQUIRED
        .iter()
        .copied()
        .filter(|lib| {
            !std::process::Command::new("pkg-config")
                .args(["--exists", lib])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        })
        .collect();

    if !missing.is_empty() {
        println!(
            "cargo:warning=missing system libraries: {}. \
             Run ./scripts/setup.sh to install the required dependencies.",
            missing.join(", ")
        );
    }
}
