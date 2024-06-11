fn main() {
    #[cfg(unix)]
    std::process::Command::new("apt")
        .args([
            "install",
            "libgtk-3-dev",
            "libxdo-dev",
            "libayatana-appindicator3-dev",
            "-y",
        ])
        .status()
        .unwrap();
}
