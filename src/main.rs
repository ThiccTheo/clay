mod action;
mod app;
mod batch;
mod id;
mod object;
mod prelude;
mod state;
mod transform;
mod world;
mod property;
mod cast;
mod as_any;
mod active;
mod visible;
mod sprite_sheet_index;

// use {
//     app::App,
//     ggez::{
//         conf::{FullscreenType, WindowMode, WindowSetup},
//         ContextBuilder,
//     },
// };

fn main() {
    // App::new(Box::new(Playing::default())).run(
    //     ContextBuilder::new("", "")
    //         .add_resource_path(format!("{}/assets", env!("CARGO_MANIFEST_DIR")))
    //         .window_setup(WindowSetup::default().title("Hi Mom!").vsync(false))
    //         .window_mode(
    //             WindowMode::default()
    //                 .dimensions(1280., 720.)
    //                 .fullscreen_type(FullscreenType::Windowed)
    //                 .borderless(true),
    //         )
    //         .build()
    //         .unwrap(),
    // );
}
