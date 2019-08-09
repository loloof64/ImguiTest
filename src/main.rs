mod initializer;
use imgui::*;

fn main() {
    let system = initializer::init(
        "Simple test de ImGui Rust",
        640f64,
        480f64
    );
    system.main_loop(|_, ui| {
        example_1(ui);
    });
}

fn example_1(ui: &Ui) {
    let w = ui.window(im_str!("Example 1: Boolean radio buttons"))
        .size([500.0, 200.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(|| {
        ui.text_wrapped(im_str!(
            "Boolean radio buttons accept a boolean active state, which is passed as a value and \
             not as a mutable reference. This means that it's not updated automatically, so you \
             can implement any click behaviour you want. The return value is true if the button \
             was clicked."
        ));
    });
}
