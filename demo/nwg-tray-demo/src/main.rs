extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::rc::Rc;
use native_windows_derive::NwgUi;
use nwg::{ControlHandle, NativeUi, RawEventHandler};
use thiserror::Error;
use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, WM_HOTKEY,
};

const LOGIN_WINDOW_SIZE: (i32, i32) = (300, 150);
const HOTKEY_ID: i32 = 123;
const VK_A: u32 = 0x41;
const HOTKEY_CALLBACK__HANDLER_ID: usize = 0x12345;

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

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control]
    window: nwg::MessageWindow, // 主窗口

    #[nwg_resource(source_bin: Some(include_bytes!("icon.png").as_slice()))]
    icon: nwg::Icon, // 图标资源

    #[nwg_control(parent: window, icon: Some(& data.icon), tip: Some("Tray"))]
    #[nwg_events(MousePressLeftUp: [App::show_menu], OnContextMenu: [App::show_menu])]
    tray: nwg::TrayNotification, // 任务栏图标

    #[nwg_control(parent: window, popup: true)]
    menu: nwg::Menu, // 菜单

    #[nwg_control(parent: menu, text: "Login")]
    #[nwg_events(OnMenuItemSelected: [App::show_login_window])]
    menu_item_login: nwg::MenuItem,

    #[nwg_control(parent: menu, text: "Greet Notification")]
    #[nwg_events(OnMenuItemSelected: [App::greet_by_notification])]
    menu_item_greet_notification: nwg::MenuItem,

    #[nwg_control(parent: menu, text: "Global Hotkey", check: false)]
    #[nwg_events(OnMenuItemSelected: [App::switch_global_hotkey])]
    menu_item_hotkey: nwg::MenuItem,

    #[nwg_control(parent: menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [App::exit])]
    menu_item_exit: nwg::MenuItem, // 退出菜单项

    // for login
    #[nwg_control(size: LOGIN_WINDOW_SIZE,
    position: get_position_of_screen_center(LOGIN_WINDOW_SIZE), title: "Login", flags: "WINDOW")]
    login_window: nwg::Window, // 登录窗口

    #[nwg_layout(parent: login_window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(parent: login_window, text: "Username: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 1, row_span: 1)]
    account_label: nwg::Label,

    #[nwg_control(parent: login_window)]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, col_span: 2, row_span: 1)]
    account_input: nwg::TextInput,

    #[nwg_control(parent: login_window, text: "Password: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 1, row_span: 1)]
    password_label: nwg::Label,

    #[nwg_control(parent: login_window, password: Some('*'))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1, col_span: 2, row_span: 1)]
    password_input: nwg::TextInput,

    #[nwg_control(parent: login_window, text: "Login")]
    #[nwg_layout_item(layout: layout, col: 0, row: 2, col_span: 3, row_span: 1)]
    #[nwg_events(OnButtonClick: [App::login])]
    login_button: nwg::Button,
}

impl HotkeyCallback for App {
    fn hotkey_fired(&self, hotkey_id: i32) -> () {
        let text = nwg::Clipboard::data_text(&self.window).unwrap_or_else(|| "(Empty)".to_string());
        nwg::modal_info_message(&self.window, &format!("Hotkey fired: {}", hotkey_id), &text);
    }
}

impl App {
    fn login(&self) {
        let account = self.account_input.text();
        let password = self.password_input.text();
        nwg::simple_message(
            "Login Info",
            &format!("Account: {}\nPassword: {}", account, password),
        );
        self.login_window.set_visible(false);
    }

    fn register_hotkey(&self) -> Result<(), Error> {
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

    fn unregister_hotkey(&self) -> Result<(), Error> {
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

    fn show_login_window(&self) {
        self.login_window.set_visible(true);
    }

    fn greet_by_notification(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray
            .show("Hello", Some("Hello World!"), Some(flags), Some(&self.icon));
    }

    fn switch_global_hotkey(&self) {
        if self.menu_item_hotkey.checked() {
            self.unregister_hotkey().unwrap();
            self.menu_item_hotkey.set_checked(false);
        } else {
            self.register_hotkey().unwrap();
            self.menu_item_hotkey.set_checked(true);
        }
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

fn get_position_of_screen_center(window_size: (i32, i32)) -> (i32, i32) {
    let x = (nwg::Monitor::width() - window_size.0) / 2;
    let y = (nwg::Monitor::height() - window_size.1) / 2;
    (x, y)
}


use app_ui::AppUi;
struct AppUiExt {
    app_ui: AppUi,
    raw_event_handlers: RefCell<Vec<RawEventHandler>>,
}

impl AppUiExt {
    fn bind_event_handler_ext(&self) {
        let weak_app = Rc::downgrade(&(self.app_ui as Rc<App>));
        let hotkey_event_handler =
            move |_hwnd: HWND, msg: UINT, wparam: WPARAM, _lparam: LPARAM| {
                if msg == WM_HOTKEY {
                    if let Some(inner_app) = weak_app.upgrade() {
                        inner_app.hotkey_fired(wparam as i32);
                    }
                }
                None
            };
        self.raw_event_handlers.borrow_mut().push(
            nwg::bind_raw_event_handler(
                &self.app_ui.window.handle,
                HOTKEY_CALLBACK__HANDLER_ID,
                hotkey_event_handler,
            )
                .unwrap(),
        );
    }
}

// //
// // ALL of this stuff is handled by native-windows-derive
// //
// mod system_tray_ui {
//
//     use std::cell::RefCell;
//     use std::ops::Deref;
//     use std::rc::Rc;
//
//     use native_windows_gui as nwg;
//     use nwg::{EventHandler, RawEventHandler};
//     use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
//     use winapi::shared::windef::HWND;
//
//     use super::*;
//
//     pub struct AppUi {
//         app: Rc<App>,
//         event_handlers: RefCell<Vec<EventHandler>>,
//         raw_event_handlers: RefCell<Vec<RawEventHandler>>,
//     }
//
//     impl NativeUi<AppUi> for App {
//         fn build_ui(mut app: App) -> std::result::Result<AppUi, nwg::NwgError> {
//             nwg::Font::set_global_family("Microsoft YaHei")?;
//
//             // 资源
//             nwg::Icon::builder()
//                 .source_bin(Some(include_bytes!("icon.png").as_slice()))
//                 .build(&mut app.icon)?;
//
//             // 主窗口，不显示
//             nwg::MessageWindow::builder().build(&mut app.window)?;
//
//             // 系统托盘
//             nwg::TrayNotification::builder()
//                 .parent(&app.window)
//                 .icon(Some(&app.icon))
//                 .tip(Some("Tray"))
//                 .build(&mut app.tray)?;
//
//             // 托盘菜单
//             nwg::Menu::builder()
//                 .popup(true)
//                 .parent(&app.window)
//                 .build(&mut app.menu)?;
//             nwg::MenuItem::builder()
//                 .text("Login")
//                 .parent(&app.menu)
//                 .build(&mut app.menu_item_login)?;
//             nwg::MenuItem::builder()
//                 .text("Greet Notification")
//                 .parent(&app.menu)
//                 .build(&mut app.menu_item_greet_notification)?;
//             nwg::MenuItem::builder()
//                 .text("Global Hotkey (Ctrl Shift Alt A)")
//                 .parent(&app.menu)
//                 .build(&mut app.menu_item_hotkey)?;
//             nwg::MenuItem::builder()
//                 .text("Exit")
//                 .parent(&app.menu)
//                 .build(&mut app.menu_item_exit)?;
//
//             // 登录窗口
//             nwg::Window::builder()
//                 .size(LOGIN_WINDOW_SIZE)
//                 .position(get_position_of_screen_center(LOGIN_WINDOW_SIZE))
//                 .title("Login")
//                 .flags(nwg::WindowFlags::WINDOW)
//                 .build(&mut app.login_window)?;
//             nwg::Label::builder()
//                 .parent(&app.login_window)
//                 .text("Username: ")
//                 .build(&mut app.account_label)?;
//             nwg::TextInput::builder()
//                 .parent(&app.login_window)
//                 .build(&mut app.account_input)?;
//             nwg::Label::builder()
//                 .parent(&app.login_window)
//                 .text("Password: ")
//                 .build(&mut app.password_label)?;
//             nwg::TextInput::builder()
//                 .parent(&app.login_window)
//                 .password(Some('*'))
//                 .build(&mut app.password_input)?;
//             nwg::Button::builder()
//                 .parent(&app.login_window)
//                 .text("Login")
//                 .build(&mut app.login_button)?;
//             // 布局
//             nwg::GridLayout::builder()
//                 .parent(&app.login_window)
//                 .spacing(5)
//                 .child_item(nwg::GridLayoutItem::new(&app.account_label, 0, 0, 1, 1))
//                 .child_item(nwg::GridLayoutItem::new(&app.account_input, 1, 0, 2, 1))
//                 .child_item(nwg::GridLayoutItem::new(&app.password_label, 0, 1, 1, 1))
//                 .child_item(nwg::GridLayoutItem::new(&app.password_input, 1, 1, 2, 1))
//                 .child_item(nwg::GridLayoutItem::new(&app.login_button, 0, 2, 3, 1))
//                 .build(&app.layout)?;
//
//             app.menu_item_hotkey.set_checked(false);
//
//             let ui = AppUi {
//                 app: Rc::new(app),
//                 event_handlers: Default::default(),
//                 raw_event_handlers: Default::default(),
//             };
//
//             let weak_app = Rc::downgrade(&ui.app);
//             let event_handler = move |event, _evt_data, handle| {
//                 if let Some(inner_app) = weak_app.upgrade() {
//                     match event {
//                         nwg::Event::OnContextMenu => {
//                             if handle == inner_app.tray {
//                                 inner_app.show_menu();
//                             }
//                         }
//                         nwg::Event::OnMenuItemSelected => {
//                             if handle == inner_app.menu_item_login {
//                                 inner_app.show_login_window();
//                             } else if handle == inner_app.menu_item_greet_notification {
//                                 inner_app.greet_by_notification();
//                             } else if handle == inner_app.menu_item_hotkey {
//                                 if inner_app.menu_item_hotkey.checked() {
//                                     inner_app.unregister_hotkey().unwrap();
//                                     inner_app.menu_item_hotkey.set_checked(false);
//                                 } else {
//                                     inner_app.register_hotkey().unwrap();
//                                     inner_app.menu_item_hotkey.set_checked(true);
//                                 }
//                             } else if handle == inner_app.menu_item_exit {
//                                 inner_app.exit();
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             };
//
//             ui.event_handlers
//                 .borrow_mut()
//                 .push(nwg::full_bind_event_handler(
//                     &ui.window.handle,
//                     event_handler,
//                 ));
//
//             let weak_app = Rc::downgrade(&ui.app);
//             let login_window_event_handler = move |event, _evt_data, handle| {
//                 if let Some(inner_app) = weak_app.upgrade() {
//                     match event {
//                         nwg::Event::OnButtonClick => {
//                             if handle == inner_app.login_button {
//                                 inner_app.login();
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             };
//
//             ui.event_handlers
//                 .borrow_mut()
//                 .push(nwg::full_bind_event_handler(
//                     &ui.login_window.handle,
//                     login_window_event_handler,
//                 ));
//
//             let weak_app = Rc::downgrade(&ui.app);
//             let hotkey_event_handler =
//                 move |_hwnd: HWND, msg: UINT, wparam: WPARAM, _lparam: LPARAM| {
//                     if msg == WM_HOTKEY {
//                         if let Some(inner_app) = weak_app.upgrade() {
//                             inner_app.hotkey_fired(wparam as i32);
//                         }
//                     }
//                     None
//                 };
//
//             ui.raw_event_handlers.borrow_mut().push(
//                 nwg::bind_raw_event_handler(
//                     &ui.window.handle,
//                     HOTKEY_CALLBACK__HANDLER_ID,
//                     hotkey_event_handler,
//                 )
//                     .unwrap(),
//             );
//
//             return Ok(ui);
//         }
//     }
//
//     impl Drop for AppUi {
//         fn drop(&mut self) {
//             let mut handlers = self.event_handlers.borrow_mut();
//             for handler in handlers.drain(0..) {
//                 nwg::unbind_event_handler(&handler);
//             }
//
//             let mut handlers = self.raw_event_handlers.borrow_mut();
//             for handler in handlers.drain(0..) {
//                 let _ = nwg::unbind_raw_event_handler(&handler); // ignore the unbind result
//             }
//         }
//     }
//
//     impl Deref for AppUi {
//         type Target = App;
//
//         fn deref(&self) -> &App {
//             &self.app
//         }
//     }
// }

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app_ui = App::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
