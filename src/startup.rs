use crate::{AppWindow, Backend};
use native_dialog::{DialogBuilder, MessageLevel};
use slint::{ComponentHandle, Weak};

pub fn init(ui_weak: Weak<AppWindow>) {
    // 启动前事件
    // 加载配置文件
    let config_file = match std::fs::read_to_string("config.txt") {
        Ok(config) => config,
        Err(_) => {
            let _ = DialogBuilder::message()
                .set_title("警告")
                .set_text("配置文件不存在，已创建默认配置文件")
                .set_level(MessageLevel::Info)
                .confirm()
                .show();

            std::fs::write("config.txt", "0").unwrap();
            "0".to_string()
        }
    };

    let number = config_file.trim().parse::<i32>().unwrap();
    let ui_weak = ui_weak.clone();
    if let Some(ui_strong) = ui_weak.upgrade() {
        let backend = ui_strong.global::<Backend>();
        backend.set_number(number);
    }
}
