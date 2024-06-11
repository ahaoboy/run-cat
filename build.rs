fn main() {
    // sudo
    #[cfg(unix)]
    std::process::Command::new("sudo")
        .args(&["apt install libgtk-3-dev libxdo-dev libappindicator3-dev #or libayatana-appindicator3-dev -y"])
        .status()
        .unwrap();
}
