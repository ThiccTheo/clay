mod action;
mod app;
mod batch;
mod id;
mod object;
mod player;
mod playing;
mod prelude;
mod state;
mod transform;
mod world;

use {
    app::App,
    ggez::{
        conf::{FullscreenType, WindowMode, WindowSetup},
        ContextBuilder,
    },
    playing::Playing,
};

fn main() {
    App::new(Box::new(Playing::default())).run(
        ContextBuilder::new("", "")
            .add_resource_path(format!("{}/assets", env!("CARGO_MANIFEST_DIR")))
            .window_setup(WindowSetup::default().title("Hi Mom!").vsync(false))
            .window_mode(
                WindowMode::default()
                    .dimensions(1280., 720.)
                    .fullscreen_type(FullscreenType::Windowed)
                    .borderless(true),
            )
            .build()
            .unwrap(),
    );
}
