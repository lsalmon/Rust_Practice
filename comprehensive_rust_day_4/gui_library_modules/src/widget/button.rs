pub use super::Label;
pub use super::Widget;

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "\n| {} |\n", self.label.label);
        //self.label.draw_into(buffer);
    }
}
