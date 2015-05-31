use std::thread;

use config::Config;
use ui::Window;

pub struct Foundry<'a> {
    window: Window<'a>,
}

impl<'a> Foundry<'a> {
    pub fn new(config: &Config) -> Foundry {
        Foundry {
            window: Window::new(config),
        }
    }

    pub fn begin(&mut self) {
        let _ = thread::scoped(move || {
            self.window.spawn_ui();
        });


        loop {
            println!("Step foundry!");
        }
    }
}
