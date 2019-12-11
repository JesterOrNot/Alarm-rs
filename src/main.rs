extern crate iui;
use iui::controls::{Button, Group, Label, VerticalBox};
use iui::prelude::*;

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Hello World", 200, 100, WindowType::HasMenubar);
    let mut vbox = VerticalBox::new(&ui);
    let label = Label::new(&ui, "\n\n   Hello World From Rust\n");
    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}
