mod action;
mod app;
mod id;
mod object;
mod player;
mod playing;
mod prelude;
mod state;
mod world;

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
            .add_resource_path(format!("{}\\assets", env!("CARGO_MANIFEST_DIR")))
            .window_setup(WindowSetup::default().title("Hi Mom!"))
            .window_mode(WindowMode::default().dimensions(1280., 720.))
            .build()
            .unwrap(),
    );
}
