A cute running cat animation on your windows taskbar.

## install

```bash
cargo install run-cat
or
cargo binstall run-cat
```

![run-cat](./assets/run-cat.gif)

## add to startup

### windows

```powershell
Copy-Item "$HOME/.cargo/bin/run-cat.exe" "$HOME/AppData/Roaming/Microsoft/Windows\Start Menu/Programs/Startup/run-cat.exe"
```

## todo
- [ ] Support linux
- [ ] Automatic detection theme color
- [ ] Optimize animation speed and CPU usage

inspired by https://github.com/Kyome22/RunCat_for_windows