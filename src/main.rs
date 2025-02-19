mod game;
mod direction;
mod coords;
mod animation;
mod menu;
mod control;
mod gameplay;

use game::{GameState, MyGamePlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_framepace::{FramepaceSettings, Limiter};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never, // Needed for itch.io web deployment
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // prevents blurry sprites
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280., 1024.),
                    ..default()
                }),
                ..default()
            })
    ); 

    // For limiting FPS
    app
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, |mut fps: ResMut<FramepaceSettings>| fps.limiter = Limiter::from_framerate(30.0));

    // {
    //     //Uncomment to see FPS logs
    //     use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //     .add_plugins(FrameTimeDiagnosticsPlugin::default())
    //     .add_plugins(LogDiagnosticsPlugin::default())
    // }

    app.add_plugins(MyGamePlugin);

    app.run();
}
