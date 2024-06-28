extern crate native_windows_gui as nwg;

use nwg::{ControlHandle, NativeUi};
use thiserror::Error;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{RegisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, WM_HOTKEY, UnregisterHotKey};

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    OsError(#[from] std::io::Error),
    #[error("{0}")]
    HotKeyParseError(String),
    #[error("Couldn't recognize \"{0}\" as a valid HotKey Code, if you feel like it should be, please report this to https://github.com/tauri-apps/global-hotkey"
    )]
    UnrecognizedHotKeyCode(String),
    #[error("Unexpected empty token while parsing hotkey: \"{0}\"")]
    EmptyHotKeyToken(String),
    #[error("Unexpected hotkey string format: \"{0}\", a hotkey should have the modifiers first and only contain one main key"
    )]
    UnexpectedHotKeyFormat(String),
    #[error("Failed to register hotkey")]
    FailedToRegister,
    #[error("Failed to unregister hotkey")]
    FailedToUnRegister,
    #[error("HotKey already registered")]
    AlreadyRegistered,
    #[error("Failed to watch media key event")]
    FailedToWatchMediaKeyEvent,
}

#[derive(Default)]
pub struct SystemTray {
    window: nwg::MessageWindow,    // 主窗口
    icon: nwg::Icon,               // 图标资源
    tray: nwg::TrayNotification,   // 任务栏图标
    tray_menu: nwg::Menu,          // 菜单
    tray_item_exit: nwg::MenuItem, // 退出菜单项
    tray_item_greet_window: nwg::MenuItem,
    tray_item_greet_notification: nwg::MenuItem,
}

const HOTKEY_ID: i32 = 123;

impl SystemTray {
    fn register_hotkey(&self) -> Result<()> {
        if let ControlHandle::Hwnd(hwnd) = self.window.handle {
            let mods = MOD_NOREPEAT | MOD_CONTROL | MOD_SHIFT | MOD_ALT;
            let result = unsafe { RegisterHotKey(hwnd, HOTKEY_ID as _, mods as _, 0x41 as _) };
            if result == 0 {
                return Err(Error::AlreadyRegistered);
            }
        } else {
            return Err(Error::FailedToRegister);
        }
        nwg::bind_raw_event_handler(&self.window.handle, 0x12345, move |_hwnd, msg, param, _l| {
            if msg == WM_HOTKEY {
                println!("Global hotkey Ctrl + Alt + Shift + A pressed!");
            }
            None
        }).unwrap();
        Ok(())
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn greet_by_window(&self) {
        nwg::modal_info_message(&self.window, "Hello", "Hello World!");
    }

    fn greet_by_notification(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray
            .show("Hello", Some("Hello World!"), Some(flags), Some(&self.icon));
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        let hwnd = self.window.handle.hwnd().unwrap() as HWND;
        unsafe {
            UnregisterHotKey(hwnd, HOTKEY_ID);
        }
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use native_windows_gui as nwg;

    use super::*;

    pub struct SystemTrayUi {
        inner: Rc<SystemTray>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl NativeUi<SystemTrayUi> for SystemTray {
        fn build_ui(mut data: SystemTray) -> std::result::Result<SystemTrayUi, nwg::NwgError> {
            // Resources
            nwg::Icon::builder()
                .source_bin(Some(include_bytes!("icon.png").as_slice()))
                .build(&mut data.icon)?;

            // Controls
            nwg::MessageWindow::builder().build(&mut data.window)?;

            nwg::TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("Tray"))
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("Greet Window")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_greet_window)?;
            nwg::MenuItem::builder()
                .text("Greet Notification")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_greet_notification)?;
            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_exit)?;

            data.register_hotkey().unwrap();

            let ui = SystemTrayUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        nwg::Event::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                SystemTray::show_menu(&evt_ui);
                            }
                        }
                        nwg::Event::OnMenuItemSelected => {
                            if &handle == &evt_ui.tray_item_exit {
                                SystemTray::exit(&evt_ui);
                            } else if &handle == &evt_ui.tray_item_greet_window {
                                SystemTray::greet_by_window(&evt_ui);
                            } else if &handle == &evt_ui.tray_item_greet_notification {
                                SystemTray::greet_by_notification(&evt_ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(nwg::full_bind_event_handler(
                    &ui.window.handle,
                    handle_events,
                ));

            return Ok(ui);
        }
    }

    impl Drop for SystemTrayUi {
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for SystemTrayUi {
        type Target = SystemTray;

        fn deref(&self) -> &SystemTray {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
