use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

pub fn main() {
    // basic();
    // radio_buttons::main();
    // check_buttons::main();
    check_buttons::set_checked();
}
pub fn basic() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello world"));

    app.run().unwrap();
}

mod radio_buttons{
    use fltk::{prelude::*, *};

pub fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    // only one can be toggled by the user at a time, the other will be automatically untoggled
    let btn1 = button::RadioRoundButton::default().with_label("Option 1");
    
    // 单选按钮独有方法：
   // btn1.clear_visible_focus();

    let btn2 = button::RadioRoundButton::default().with_label("Option 2"); 
    flex.end();
    win.end();
    win.show();
    a.run().unwrap();
}
}

mod check_buttons{
    /**
     * CheckButton also provides a convenience method is_checked(),
     *  while radio buttons provide an is_toggled().
     * */
    use fltk::{prelude::*, *};

pub fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    let btn1 = button::CheckButton::default().with_label("Option 1");
    let btn2 = button::CheckButton::default().with_label("Option 2");
    let mut btn3 = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    btn3.set_callback(move |btn3| {
        if btn1.value() {
            println!("btn1 is checked");
        }
        if btn2.value() {
            println!("btn2 is checked");
        }
    });

    a.run().unwrap();
}

 pub fn set_checked(){
    fn main() {
        let a = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
        let mut btn1 = button::CheckButton::default().with_label("Option 1");
        btn1.set_value(true);
        // Similarly you can use btn1.set_checked(true)
        let btn2 = button::CheckButton::default().with_label("Option 2");
        let mut btn3 = button::Button::default().with_label("Submit");
        flex.end();
        win.end();
        win.show();
    
        btn3.set_callback(move |btn3| {
            if btn1.value() {
                println!("btn1 is checked");
            }
            if btn2.value() {
                println!("btn2 is checked");
            }
        });
    
        a.run().unwrap();
    }
    // 懒得拷贝了 直接再函数中调用main了
    main();
 }

}
