use std::cell::Cell;

use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button,
};

fn main() {
    let app: Application = Application::builder()
        .application_id("io.github.aerphanas")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    const WW: i32 = 400;
    const WH: i32 = 200;

    let number: Cell<u32> = Cell::new(0);

    let button: Button = Button::builder()
        .label("Click me")
        .margin_start(50)
        .margin_end(50)
        .margin_top((WH / 2) + 10)
        .margin_bottom((WH / 2) + 10)
        .build();

    button.connect_clicked(move |button| {
        let n: u32 = number.get() + 1;
        let label: String = format!("Clicked {}", n);
        button.set_label(&label);
        number.set(n);
    });

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("Counter")
        .child(&button)
        .default_width(WW)
        .default_height(WH)
        .build();

    window.present();
}
