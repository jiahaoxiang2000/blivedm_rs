use imgui::*;

mod support;
mod bili_live_dm;

fn main() {
    let system = support::init(file!());

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];

    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                ui.text_wrapped("德意志的科学技术世界第一");
                ui.text_wrapped("术术叔叔");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
