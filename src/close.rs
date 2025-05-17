use std::{fs::File, io::Write};

use crate::{AppWindow, Backend};
use slint::{CloseRequestResponse, ComponentHandle, Weak};

pub fn close_windows_event(ui_weak: Weak<AppWindow>) {
    if let Some(ui_strong) = ui_weak.upgrade() {
        ui_strong.window().on_close_requested({
            let ui_weak = ui_weak.clone();
            move || {
                if let Some(ui_strong) = ui_weak.upgrade() {
                    let backend = ui_strong.global::<Backend>();
                    let number = backend.get_number();

                    // 把 number 写入 config.txt
                    let mut file = File::create("config.txt").unwrap();
                    writeln!(file, "{}", number).unwrap();
                }
                CloseRequestResponse::HideWindow
            }
        });
    }
}
