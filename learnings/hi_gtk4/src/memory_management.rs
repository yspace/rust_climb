use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, Orientation};

use gtk::glib;

pub fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::new(application);

    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    // let mut number = 0;

    // // Connect callbacks
    // // When a button is clicked, `number` should be changed
    // button_increase.connect_clicked(|_| number += 1);
    // button_decrease.connect_clicked(|_| number -= 1);
    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));

    //  // Connect callbacks, when a button is clicked `number` will be changed
    //  let number_copy = number.clone();
    //  button_increase.connect_clicked(move |_| number_copy.set(number_copy.get() + 1));
    //  button_decrease.connect_clicked(move |_| number.set(number.get() - 1));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    // button_increase.connect_clicked(clone!(@weak number, @strong button_decrease =>
    //     move |_| {
    //         number.set(number.get() + 1);
    //         button_decrease.set_label(&number.get().to_string());
    // }));
    // button_decrease.connect_clicked(clone!(@strong button_increase =>
    //     move |_| {
    //         number.set(number.get() - 1);
    //         button_increase.set_label(&number.get().to_string());
    // }));

    // Add buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
