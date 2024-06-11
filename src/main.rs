#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
#![allow(unused)]

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

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

const FPS_STEPS: [(u64, u64); 5] = [(20, 10), (40, 15), (60, 20), (80, 25), (100, 30)];

fn get_fps(cpu: f32) -> u64 {
    for (percent, fps) in FPS_STEPS {
        if cpu <= percent as f32 {
            return fps;
        }
    }
    return 30;
}

fn main() {
    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        let icon = load_icon(std::path::Path::new(path));

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_icon(icon)
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

    tray_menu.append_items(&[&dark_item, &light_item, &quit_item]);

    let app_icon = load_icon(include_bytes!("../assets/png/app32.png"));
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("run cat~")
        .with_icon(app_icon)
        .build()
        .unwrap();

    let event_loop = EventLoop::new().unwrap();
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let mut sys = System::new();
    sys.refresh_all();
    let mut last_update_ts = std::time::Instant::now();

    let mut avg_cpu_usage = 0;
    let mut last_refresh_cpu = std::time::Instant::now();
    let cpu_gap = 300;
    let mut last_avg_cpu_usage = 0.;

    event_loop.run(move |event, event_loop| {
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
        let sleep_time = std::time::Duration::from_millis(ms);

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
    });
}
