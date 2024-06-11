#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};
use winit::event_loop::{ControlFlow, EventLoop};

use sysinfo::System;

fn load_icon(buf: &[u8]) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(buf)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

const FPS_STEPS: [(u64, u64); 5] = [(20, 5), (40, 10), (60, 15), (80, 20), (100, 25)];

fn get_fps(cpu: f32) -> u64 {
    for (percent, fps) in FPS_STEPS {
        if cpu <= percent as f32 {
            return fps;
        }
    }
    30
}

fn main() {
    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    let app_icon = load_icon(include_bytes!("../assets/png/app32.png"));
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;
        let app_icon = load_icon(include_bytes!("../assets/png/app32.png"));
        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_icon(app_icon.clone())
            .build()
            .unwrap();

        gtk::main();
    });

    let dark_cat_icons: [tray_icon::Icon; 5] = [
        load_icon(include_bytes!("../assets/png/dark_cat_0.png")),
        load_icon(include_bytes!("../assets/png/dark_cat_1.png")),
        load_icon(include_bytes!("../assets/png/dark_cat_2.png")),
        load_icon(include_bytes!("../assets/png/dark_cat_3.png")),
        load_icon(include_bytes!("../assets/png/dark_cat_4.png")),
    ];

    let light_cat_icons: [tray_icon::Icon; 5] = [
        load_icon(include_bytes!("../assets/png/light_cat_0.png")),
        load_icon(include_bytes!("../assets/png/light_cat_1.png")),
        load_icon(include_bytes!("../assets/png/light_cat_2.png")),
        load_icon(include_bytes!("../assets/png/light_cat_3.png")),
        load_icon(include_bytes!("../assets/png/light_cat_4.png")),
    ];

    let tray_menu = Menu::new();
    // tray_menu.append_items("items");
    let mut c = 0;

    let mut theme = "dark";

    let quit_item = MenuItem::new("quit", true, None);
    let dark_item = MenuItem::new("dark", true, None);
    let light_item = MenuItem::new("light", true, None);

    tray_menu
        .append_items(&[&dark_item, &light_item, &quit_item])
        .unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("run cat~")
        .with_icon(app_icon.clone())
        .build()
        .unwrap();

    let event_loop = EventLoop::new().unwrap();
    let menu_channel = MenuEvent::receiver();
    // let tray_channel = TrayIconEvent::receiver();

    let mut sys = System::new();
    sys.refresh_all();
    let mut last_update_ts = std::time::Instant::now();

    let mut last_refresh_cpu = std::time::Instant::now();
    let cpu_gap = 300;
    let mut last_avg_cpu_usage = 0.;

    event_loop
        .run(move |_, event_loop| {
            // We add delay of 16 ms (60fps) to event_loop to reduce cpu load.
            // This can be removed to allow ControlFlow::Poll to poll on each cpu cycle
            // Alternatively, you can set ControlFlow::Wait or use TrayIconEvent::set_event_handler,
            // see https://github.com/tauri-apps/tray-icon/issues/83#issuecomment-1697773065
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                std::time::Instant::now() + std::time::Duration::from_millis(16),
            ));

            if let Ok(event) = menu_channel.try_recv() {
                if event.id == quit_item.id() {
                    std::process::exit(0);
                }
                if event.id == dark_item.id() {
                    theme = "dark";
                }
                if event.id == light_item.id() {
                    theme = "light";
                }
            }

            let now = std::time::Instant::now();

            sys.refresh_cpu();
            let avg = if last_refresh_cpu.elapsed().as_millis() < cpu_gap {
                last_avg_cpu_usage
            } else {
                last_refresh_cpu = now;
                let avg =
                    sys.cpus().iter().map(|i| i.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
                last_avg_cpu_usage = avg;
                avg
            };

            let fps: u64 = get_fps(avg);
            let ms = 1000 / fps;

            if last_update_ts.elapsed().as_millis() >= ms.into() {
                last_update_ts = now;
                let list = match theme {
                    "dark" => &dark_cat_icons,
                    "light" => &light_cat_icons,
                    _ => &dark_cat_icons,
                };
                let icon = &list[c % dark_cat_icons.len()];
                c += 1;
                tray_icon.set_icon(Some(icon.clone())).unwrap();
            }
        })
        .unwrap();
}
