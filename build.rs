#[cfg(unix)]
fn main() {
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

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./assets/ico/app.ico");
        res.compile().unwrap();
    }
}
