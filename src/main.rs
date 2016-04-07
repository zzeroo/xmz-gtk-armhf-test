extern crate gtk;

use gtk::prelude::*;
use gtk::{ Window, WindowPosition, WindowType };




fn main() {
    if gtk::init().is_err() {
        println!("Failed to initalize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("xMZ-Mod-Touch v2.0.0");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    window.show_all();
    gtk::main();
}
