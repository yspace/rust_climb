use fltk::{prelude::*, *};

pub fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    let flex = group::Flex::default()
        .with_size(100, 200)
        .column()
        .center_of_parent();

    // 该框架没有label 用frame可以替代
    let lbl1 = frame::Frame::default().with_label("labels ");

    // 惯用法
    let btn = button::Button::default().with_label("Click");
    // or
    let mut btn = button::Button::default();

    btn.set_label("@+92->"); // 格式化字符 参考 https://fltk-rs.github.io/fltk-book/Labels.html
                             // 符号 和 文本可以合并作为label的显示 但符号必须作为结尾 或者 开头存在

    // win.add(&lbl1);

    flex.end();
    win.end();
    win.show();
    a.run().unwrap();
}
