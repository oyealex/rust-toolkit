extern crate native_windows_gui as nwg;

use nwg::{ControlHandle, NativeUi};
use thiserror::Error;
use winapi::um::winuser::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, WM_HOTKEY,
};

pub type Result<T> = std::result::Result<T, Error>;

trait HotkeyCallback {
    fn hotkey_fired(&self, hotkey_id: i32) -> ();
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    OsError(#[from] std::io::Error),
    #[error("Failed to register hotkey: {0:?}")]
    FailedToRegister(String),
    #[error("Failed to unregister hotkey: {0:?}")]
    FailedToUnRegister(String),
    #[error("HotKey already registered")]
    AlreadyRegistered,
}

#[derive(Default)]
pub struct App {
    window: nwg::MessageWindow,  // 主窗口
    icon: nwg::Icon,             // 图标资源
    tray: nwg::TrayNotification, // 任务栏图标
    menu: nwg::Menu,             // 菜单
    menu_item_greet_window: nwg::MenuItem,
    menu_item_greet_notification: nwg::MenuItem,
    menu_item_hotkey: nwg::MenuItem,
    menu_item_exit: nwg::MenuItem, // 退出菜单项
}

const HOTKEY_ID: i32 = 123;
const VK_A: u32 = 0x41;

const HOTKEY_CALLBACK__HANDLER_ID: usize = 0x12345;

impl HotkeyCallback for App {
    fn hotkey_fired(&self, hotkey_id: i32) -> () {
        println!("hotkey fired: {}", hotkey_id);
    }
}

impl App {
    fn register_hotkey(&self) -> Result<()> {
        if let ControlHandle::Hwnd(hwnd) = self.window.handle {
            let mods = MOD_NOREPEAT | MOD_CONTROL | MOD_SHIFT | MOD_ALT;
            let result = unsafe { RegisterHotKey(hwnd, HOTKEY_ID as _, mods as _, VK_A as _) };
            if result == 0 {
                return Err(Error::AlreadyRegistered);
            }
        } else {
            return Err(Error::FailedToRegister(format!(
                "invalid handle: {:?}",
                self.window.handle
            )));
        }
        Ok(())
    }

    fn unregister_hotkey(&self) -> Result<()> {
        if let ControlHandle::Hwnd(hwnd) = self.window.handle {
            let result = unsafe { UnregisterHotKey(hwnd, HOTKEY_ID as _) };
            if result == 0 {
                return Err(Error::FailedToUnRegister(format!("code: {}", result)));
            }
        } else {
            return Err(Error::FailedToUnRegister(format!(
                "invalid handle: {:?}",
                self.window.handle
            )));
        }
        Ok(())
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.menu.popup(x, y);
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

impl Drop for App {
    fn drop(&mut self) {
        if let ControlHandle::Hwnd(hwnd) = self.window.handle {
            unsafe {
                UnregisterHotKey(hwnd, HOTKEY_ID);
            }
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

    use super::*;
    use native_windows_gui as nwg;
    use nwg::{EventHandler, RawEventHandler};
    use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
    use winapi::shared::windef::HWND;

    pub struct AppUi {
        app: Rc<App>,
        event_handlers: RefCell<Vec<EventHandler>>,
        raw_event_handlers: RefCell<Vec<RawEventHandler>>,
    }

    impl NativeUi<AppUi> for App {
        fn build_ui(mut app: App) -> std::result::Result<AppUi, nwg::NwgError> {
            // Resources
            nwg::Icon::builder()
                .source_bin(Some(include_bytes!("icon.png").as_slice()))
                .build(&mut app.icon)?;

            // Controls
            nwg::MessageWindow::builder().build(&mut app.window)?;

            nwg::TrayNotification::builder()
                .parent(&app.window)
                .icon(Some(&app.icon))
                .tip(Some("Tray"))
                .build(&mut app.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&app.window)
                .build(&mut app.menu)?;

            nwg::MenuItem::builder()
                .text("Greet Window")
                .parent(&app.menu)
                .build(&mut app.menu_item_greet_window)?;
            nwg::MenuItem::builder()
                .text("Greet Notification")
                .parent(&app.menu)
                .build(&mut app.menu_item_greet_notification)?;
            nwg::MenuItem::builder()
                .text("Global Hotkey (Ctrl Shift Alt A)")
                .parent(&app.menu)
                .build(&mut app.menu_item_hotkey)?;
            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&app.menu)
                .build(&mut app.menu_item_exit)?;

            app.menu_item_hotkey.set_checked(false);

            let ui = AppUi {
                app: Rc::new(app),
                event_handlers: Default::default(),
                raw_event_handlers: Default::default(),
            };

            let weak_app = Rc::downgrade(&ui.app);
            let event_handler = move |event, _evt_data, handle| {
                if let Some(inner_app) = weak_app.upgrade() {
                    match event {
                        nwg::Event::OnContextMenu => {
                            if handle == inner_app.tray {
                                inner_app.show_menu();
                            }
                        }
                        nwg::Event::OnMenuItemSelected => {
                            if handle == inner_app.menu_item_greet_window {
                                inner_app.greet_by_window();
                            } else if handle == inner_app.menu_item_greet_notification {
                                inner_app.greet_by_notification();
                            } else if handle == inner_app.menu_item_hotkey {
                                if inner_app.menu_item_hotkey.checked() {
                                    inner_app.unregister_hotkey().unwrap();
                                    inner_app.menu_item_hotkey.set_checked(false);
                                } else {
                                    inner_app.register_hotkey().unwrap();
                                    inner_app.menu_item_hotkey.set_checked(true);
                                }
                            } else if handle == inner_app.menu_item_exit {
                                inner_app.exit();
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.event_handlers
                .borrow_mut()
                .push(nwg::full_bind_event_handler(
                    &ui.window.handle,
                    event_handler,
                ));

            let weak_app = Rc::downgrade(&ui.app);
            let hotkey_event_handler =
                move |_hwnd: HWND, msg: UINT, wparam: WPARAM, _lparam: LPARAM| {
                    if msg == WM_HOTKEY {
                        if let Some(inner_app) = weak_app.upgrade() {
                            inner_app.hotkey_fired(wparam as i32);
                        }
                    }
                    None
                };

            ui.raw_event_handlers.borrow_mut().push(
                nwg::bind_raw_event_handler(
                    &ui.window.handle,
                    HOTKEY_CALLBACK__HANDLER_ID,
                    hotkey_event_handler,
                )
                .unwrap(),
            );

            return Ok(ui);
        }
    }

    impl Drop for AppUi {
        fn drop(&mut self) {
            let mut handlers = self.event_handlers.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }

            let mut handlers = self.raw_event_handlers.borrow_mut();
            for handler in handlers.drain(0..) {
                let _ = nwg::unbind_raw_event_handler(&handler); // ignore the unbind result
            }
        }
    }

    impl Deref for AppUi {
        type Target = App;

        fn deref(&self) -> &App {
            &self.app
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app_ui = App::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
