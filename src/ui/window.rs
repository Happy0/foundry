use conrod;
use conrod::{
    Theme
};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{WindowSettings, Size};
use std::path::Path;
use config::Config;

type Ui = conrod::Ui<GlyphCache<'static>>;

pub struct Window<'a> {
    config: &'a Config
}

impl<'a> Window<'a> {
    pub fn new(config: &Config) -> Window {
        Window {
            config: config,
        }
    }

    pub fn spawn_ui(&mut self) {
        let opengl = OpenGL::_3_2;
        let window = GlutinWindow::new(opengl,
                                       WindowSettings::new("Foundry".to_string(),
                                       Size {
                                           width: self.config.session.width,
                                           height: self.config.session.height,
                                       })
                                       .samples(4));

        let event_iter = window.events().ups(60).max_fps(60);
        let mut gl = GlGraphics::new(opengl);
        let font_path = Path::new("./assets/fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = GlyphCache::new(&font_path).unwrap();
        let mut ui = Ui::new(glyph_cache, theme);

        for event in event_iter {
            ui.handle_event(&event);
            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |c, gl| {
                    ui.draw(c, gl);
                });
            }
        }
    }
}
