use crate::{AppWindow, Backend};
use slint::{ComponentHandle, Weak};

pub fn init(ui_weak: Weak<AppWindow>) {
    // 启动前事件
    // 加载配置文件
    let number = std::fs::read_to_string("config.txt")
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let ui_weak = ui_weak.clone();
    if let Some(ui_strong) = ui_weak.upgrade() {
        let backend = ui_strong.global::<Backend>();
        backend.set_number(number);
    }
}
