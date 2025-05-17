use crate::{AppWindow, Backend};
use slint::{ComponentHandle, Weak};

pub fn impl_logic_for_backend(ui_weak: Weak<AppWindow>) {
    if let Some(strong_ui) = ui_weak.clone().upgrade() {
        let backend = strong_ui.global::<Backend>();

        backend.on_add({
            let ui_weak = ui_weak.clone();
            move || {
                if let Some(strong_ui) = ui_weak.upgrade() {
                    let backend = strong_ui.global::<Backend>();

                    backend.set_number(backend.get_number() + 1);
                }
            }
        });

        backend.on_sub({
            let ui_weak = ui_weak.clone();
            move || {
                if let Some(strong_ui) = ui_weak.upgrade() {
                    let backend = strong_ui.global::<Backend>();

                    backend.set_number(backend.get_number() - 1);
                }
            }
        });
    }
}
