fn main() {
    #[cfg(unix)]
    std::process::Command::new("apt")
        .args([
            "install",
            "libgtk-3-dev",
            "libxdo-dev",
            "libpango1.0-dev",
            "libayatana-appindicator3-dev",
            "-y",
        ])
        .status()
        .unwrap();
}
