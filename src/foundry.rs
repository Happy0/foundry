use conrod::{Colorable, Label, Positionable, Widget};
use conrod::color::{self, rgb};

use super::Ui;

pub struct Foundry {
    test_text: &'static str,
}

impl Foundry {
    pub fn new() -> Foundry {
        Foundry {
            test_text: "I am an ide, i promise!",
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        Label::new(self.test_text)
            .xy(50.0,50.0)
            .font_size(32)
            .color(color::rgb(1.0,0.0,0.0))
            .set(10, ui)
    }
}
