//!
//! A simple demonstration of how to make a Textbox that only accepts given characters
//!

extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Colorable, Positionable, Sizeable, TextBox, Theme, Ui, Widget};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{Events, EventLoop};
use piston::input::{RenderEvent};
use piston::window::{WindowSettings, Size};

fn main() {

    let opengl = OpenGL::V3_2;
    let window: GlutinWindow =
        WindowSettings::new(
            "Filtered Textbox Example".to_string(),
            Size { width: 200, height: 100 }
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .into();
    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut only_lowercase = String::new();

    let mut only_numbers = String::new();

    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, gl| {

                // Draw the background.
                Background::new().rgb(0.2, 0.25, 0.4).draw(ui, gl);

                // Draw the button and increment count if pressed..
                TextBox::new(&mut only_lowercase).font_size(20)
                    .dimensions(320.0, 40.0)
                    .react(|_string: &mut String|{})
                    .filter(|_string: &mut String, ch: char| {
                        ch.is_lowercase() || ch == ' '
                    }).set(0, ui);

                TextBox::new(&mut only_numbers).down_from(0, 30.0).font_size(20)
                    .dimensions(320.0, 40.0)
                    .react(|_string: &mut String|{})
                    .filter(|_string: &mut String, ch: char| {
                        ch.is_digit(10) || ch == ' '
                    }).set(1, ui);

                // Draw our Ui!
                ui.draw(c, gl);

            });
        }
    }

}
