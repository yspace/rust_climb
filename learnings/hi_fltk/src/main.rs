// use fltk::{
//     app,
//     prelude::{GroupExt, WidgetBase, WidgetExt},
//     window,
// };

use fltk::{prelude::*, *};

mod buttons;
mod labels;

fn main() {
    // run() ;
    // buttons::main();
    labels::main();
    println!("application exit!");
}

fn run() {
    // let app = app::App::default();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::new(100, 100, 400, 300, "My Window");
    wind.end();
    wind.show();
    // 内嵌窗口
    // embeded_windows();

    let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    wind.add(&but);

    // builder pattern
    let but1 = button::Button::default()
        .with_pos(10, 10)
        .with_size(80, 40)
        .with_label("Button 1");
    wind.add(&but1);

    app.run().expect("Couldn't run app");
}

fn embeded_windows() {
    let mut my_window = window::Window::new(100, 100, 400, 300, "My outter Window");
    my_window.set_color(enums::Color::Green);

    let mut my_window2 = window::Window::new(10, 10, 380, 280, "inner win");
    // 内嵌窗口 弄个黑颜色 这样便于区别
    my_window2.set_color(enums::Color::Black);
    my_window2.end();
    my_window.end();
    //  不要边 没法关了！
    // my_window.set_border(false);
    my_window.show();
}
