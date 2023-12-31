mod widget;
use crate::widget::Widget;
use crate::widget::Label;
use crate::widget::Button;
use crate::widget::Window;

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
