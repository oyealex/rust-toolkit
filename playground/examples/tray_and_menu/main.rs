#![windows_subsystem = "windows"]

use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::Context;
use anyhow::Result;
use tray_icon::menu::accelerator::{Accelerator, Code, Modifiers};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::TrayIconBuilder;
use winit::event::Event::NewEvents;
use winit::event::StartCause;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    let menu_channel = MenuEvent::receiver();
    let event_loop = EventLoop::new().with_context(|| "failed to build event loop".to_string())?;
    let mut tray_icon = None;
    let mut exit_item_id = None;

    #[allow(deprecated)]
    event_loop.run(|event, event_loop| {
        // 设置重新检查事件的间隔时间，每秒钟检查10次，避免过多消耗CPU
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(100),
        ));

        if let NewEvents(StartCause::Init) = event {
            let menu = Menu::new();
            let exit_menu = MenuItem::new(
                "E&xit",
                true,
                Some(Accelerator::new(Some(Modifiers::ALT), Code::KeyE)),
            );
            menu.append(&exit_menu).unwrap();
            exit_item_id = Some(exit_menu.id().clone());
            tray_icon = Some(
                TrayIconBuilder::new()
                    .with_menu(Box::new(menu))
                    .with_tooltip("A tray with menu")
                    .with_icon(load_icon().unwrap())
                    .with_title("Tray")
                    .build()
                    .unwrap(),
            );
        }

        if let Ok(event) = menu_channel.try_recv() {
            if let Some(menu_id) = &exit_item_id {
                if menu_id == &event.id {
                    event_loop.exit();
                }
            }
        }
    })?;
    Ok(())
}

fn load_icon() -> Result<tray_icon::Icon> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/tray_and_menu/res/icon.png"
    );
    let image = image::open(Path::new(path))
        .with_context(|| "failed to load icon file".to_string())?
        .into_rgba8();
    let (width, height) = image.dimensions();
    tray_icon::Icon::from_rgba(image.into_raw(), width, height)
        .with_context(|| "failed dto load icon data".to_string())
}
