mod direction;
mod coords;
mod animation;
mod menu;
mod control;
mod level;

use animation::play_animations;
use level::arena::setup_arena;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_rapier2d::prelude::*;
use control::handle_camera_zoom;
use level::enemy::{execute_enemy_behavior, setup_enemies, spawn_enemies};
use menu::{end_menu::{handle_restart_game, setup_end_screen}, in_game_menu::{setup_game_ui, update_game_ui}, start_menu::{handle_start_game, setup_start_screen}};
use control::init_cursor;
use level::player::{execute_bullets_move, execute_player_behavior, setup_player};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins
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
    app.add_plugins(bevy_framepace::FramepacePlugin);

    // Adding Rapier physics
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default()); // Uncomment to see collider boxes

    // {
    //     //Uncomment to see FPS logs
    //     use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //     .add_plugins(FrameTimeDiagnosticsPlugin::default())
    //     .add_plugins(LogDiagnosticsPlugin::default())
    // }

    // Game related systems
    app
        .init_state::<AppState>()

        .add_systems(Startup, |mut fps: ResMut<FramepaceSettings>| fps.limiter = Limiter::from_framerate(30.0))
        .add_systems(Startup, init_cursor)
        .add_systems(Startup, setup_camera)

        .add_systems(Startup, setup_start_screen.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_start_game.run_if(in_state(AppState::MainMenu)))

        .add_systems(OnEnter(AppState::InGame), 
            (
                setup_arena,
                setup_player,
                setup_enemies,
                setup_game_ui,
            ).chain()
        )

        .add_systems(OnEnter(AppState::End), setup_end_screen)
        .add_systems(Update, handle_restart_game.run_if(in_state(AppState::End)))

        .add_systems(Update, 
            (
                execute_player_behavior,
                execute_enemy_behavior,
                execute_bullets_move,
                spawn_enemies,
                play_animations,
                handle_camera_zoom,
                update_game_ui
            ).run_if(in_state(AppState::InGame))
        )
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    End,
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::ONE * 0.2),
    ));
}
