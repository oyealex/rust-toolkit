#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::rc::Rc;

use native_windows_derive::NwgUi;
use nwg::{ControlHandle, Font, NativeUi, RawEventHandler};
use thiserror::Error;
use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, WM_HOTKEY,
};

use app_ui::AppUi;

const LOGIN_WINDOW_SIZE: (i32, i32) = (300, 200);
const HOTKEY_ID: i32 = 123;
const VK_A: u32 = 0x41;
const HOTKEY_CALLBACK__HANDLER_ID: usize = 0x12345;
const HOTKEY_ACTIVATED_TRAY_TIPS: &'static str = "Nwg Tray Demo - Hotkey Activated";
const TRAY_TIPS: &'static str = "Nwg Tray Demo";

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

    #[nwg_resource(source_bin: Some(include_bytes!("icon-normal.ico").as_slice()),
                   size: Some((16u32, 16u32)))]
    normal_tray_icon: nwg::Icon, // 图标资源

    #[nwg_resource(source_bin: Some(include_bytes!("icon-activated.ico").as_slice()),
                   size: Some((16u32, 16u32)))]
    activated_tray_icon: nwg::Icon, // 图标资源

    #[nwg_resource(source_bin: Some(include_bytes!("icon-setting.ico").as_slice()),
                   size: Some((32u32, 32u32)))]
    setting_icon: nwg::Icon, // 图标资源

    #[nwg_control(parent: window, icon: Some(&data.normal_tray_icon), tip: Some(TRAY_TIPS))]
    #[nwg_events(MousePressLeftUp: [App::show_menu], OnContextMenu: [App::show_menu])]
    tray: nwg::TrayNotification, // 任务栏图标

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu, // 菜单

    #[nwg_control(parent: tray_menu, text: "Setting")]
    #[nwg_events(OnMenuItemSelected: [App::show_setting_window])]
    setting_menu_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Global Hotkey (Ctrl Shift Alt A)", check: false)]
    #[nwg_events(OnMenuItemSelected: [App::switch_global_hotkey])]
    hotkey_menu_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [App::exit])]
    exit_menu_item: nwg::MenuItem, // 退出菜单项

    // Setting Window
    #[nwg_control(size: LOGIN_WINDOW_SIZE,
                  position: get_position_of_screen_center(LOGIN_WINDOW_SIZE),
                  title: "Setting",
                  icon: Some(&data.setting_icon),
                  flags: "WINDOW")]
    setting_window: nwg::Window, // 配置窗口

    #[nwg_layout(parent: setting_window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(parent: setting_window, text: "Username: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 1, row_span: 1)]
    account_label: nwg::Label,

    #[nwg_control(parent: setting_window)]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, col_span: 2, row_span: 1)]
    account_input: nwg::TextInput,

    #[nwg_control(parent: setting_window, text: "Password: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 1, row_span: 1)]
    password_label: nwg::Label,

    #[nwg_control(parent: setting_window, password: Some('*'))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1, col_span: 2, row_span: 1)]
    password_input: nwg::TextInput,

    #[nwg_control(parent: setting_window,
                  text: "Save &token to file",
                  check_state: CheckBoxState::Unchecked,
                  flags: "VISIBLE|TAB_STOP")]
    #[nwg_layout_item(layout: layout, col: 0, row: 2, col_span: 2, row_span: 1)]
    save_token_checkbox: nwg::CheckBox,

    #[nwg_control(parent: setting_window, text: "&Show", focus: true)]
    #[nwg_layout_item(layout: layout, col: 2, row: 2, col_span: 1, row_span: 1)]
    show_config_button: nwg::Button,

    #[nwg_control(parent: setting_window, text: "&Login", focus: true)]
    #[nwg_layout_item(layout: layout, col: 0, row: 3, col_span: 3, row_span: 1)]
    #[nwg_events(OnButtonClick: [App::login])]
    login_button: nwg::Button,
}

impl HotkeyCallback for App {
    fn hotkey_fired(&self, hotkey_id: i32) -> () {
        if let Some(text) = nwg::Clipboard::data_text(&self.window) {
            nwg::modal_info_message(&self.window, &format!("Hotkey fired: {}", hotkey_id), &text);
        } else {
            nwg::modal_info_message(
                &self.window,
                &format!("Hotkey fired: {}", hotkey_id),
                "no text content found in clipboard",
            );
        }
    }
}

impl App {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn _greet_by_notification(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(
            "Hello",
            Some("Hello World!"),
            Some(flags),
            Some(&self.normal_tray_icon),
        );
    }

    fn switch_global_hotkey(&self) {
        if self.hotkey_menu_item.checked() {
            self.unregister_hotkey().unwrap();
            self.hotkey_menu_item.set_checked(false);
            self.tray.set_icon(&self.normal_tray_icon);
            self.tray.set_tip(TRAY_TIPS);
        } else {
            self.register_hotkey().unwrap();
            self.hotkey_menu_item.set_checked(true);
            self.tray.set_icon(&self.activated_tray_icon);
            self.tray.set_tip(HOTKEY_ACTIVATED_TRAY_TIPS);
        }
    }

    fn show_setting_window(&self) {
        let position = get_position_of_screen_center(LOGIN_WINDOW_SIZE);
        self.setting_window.set_position(position.0, position.1);
        self.setting_window.set_visible(true);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn login(&self) {
        let account = self.account_input.text();
        let password = self.password_input.text();
        nwg::simple_message(
            "Login Info",
            &format!("Account: {}\nPassword: {}", account, password),
        );
        self.account_input.set_text("");
        self.password_input.set_text("");
        self.setting_window.set_visible(false);
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

struct AppUiWrapper {
    app_ui: Rc<AppUi>,
    raw_event_handlers: RefCell<Vec<RawEventHandler>>,
}

impl AppUiWrapper {
    fn bind_event_handler_ext(&self) {
        let weak_app_ui = Rc::downgrade(&self.app_ui);
        let hotkey_event_handler =
            move |_hwnd: HWND, msg: UINT, wparam: WPARAM, _lparam: LPARAM| {
                if msg == WM_HOTKEY {
                    if let Some(inner_app) = weak_app_ui.upgrade() {
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

impl Drop for AppUiWrapper {
    fn drop(&mut self) {
        let mut handlers = self.raw_event_handlers.borrow_mut();
        for handler in handlers.drain(0..) {
            let _ = nwg::unbind_raw_event_handler(&handler); // ignore unbind result
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    Font::set_global_family("Microsoft YaHei").expect("Failed to setup global font family");
    let app_ui = App::build_ui(Default::default()).expect("Failed to build UI");
    let app_ui_wrapper = AppUiWrapper {
        app_ui: Rc::new(app_ui),
        raw_event_handlers: Default::default(),
    };
    app_ui_wrapper.bind_event_handler_ext();
    nwg::dispatch_thread_events();
}
