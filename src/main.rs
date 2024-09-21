mod action;
mod app;
mod object;
mod playing;
mod prelude;
mod state;
mod world;
mod player;

use {
    app::App,
    ggez::{
        conf::{WindowMode, WindowSetup},
        ContextBuilder,
    },
    playing::Playing,
};

fn main() {
    App::new(Box::new(Playing::default())).run(
        ContextBuilder::new("", "")
            .window_setup(WindowSetup::default().title("Hi Mom!"))
            .window_mode(WindowMode::default().dimensions(1280., 720.))
            .build()
            .unwrap(),
    );
}
