extern crate native_windows_gui as nwg;

use nwg::NativeUi;

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

impl SystemTray {
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

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct SystemTrayUi {
        inner: Rc<SystemTray>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl NativeUi<SystemTrayUi> for SystemTray {
        fn build_ui(mut data: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
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
