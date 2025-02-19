use bevy::prelude::*;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};

use crate::{control::init_cursor, gameplay::MyGameplayPlugin, menu::{end_menu::{handle_restart_game, setup_end_screen}, start_menu::{handle_start_game, setup_start_screen}}};

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    End,
}

pub struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        // Adding Rapier physics
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0).in_fixed_schedule());

        // https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html
        app.insert_resource(Time::<Fixed>::from_hz(64.0));

        #[cfg(debug_assertions)]
        app.add_plugins(RapierDebugRenderPlugin::default()); // Uncomment to see collider boxes


        app.add_plugins(MyGameplayPlugin);

        app
            .init_state::<GameState>()
    
            .add_systems(Startup, init_cursor)
            .add_systems(Startup, setup_camera)
    
            .add_systems(Startup, setup_start_screen.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, handle_start_game.run_if(in_state(GameState::MainMenu)))
    
            .add_systems(OnEnter(GameState::End), setup_end_screen)
            .add_systems(Update, handle_restart_game.run_if(in_state(GameState::End)));
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::ONE * 0.2),
    ));
}
