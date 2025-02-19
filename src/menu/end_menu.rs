use bevy::prelude::*;

use crate::{gameplay::{GameScore, LevelComponents}, GameState};

#[derive(Component)]
pub struct EndScreenElement;

pub fn setup_end_screen(
    mut commands: Commands,
    score: Res<GameScore>,
    level_entities: Query<Entity, With<LevelComponents>>,
) {
    for level_entity in &level_entities {
        commands.entity(level_entity).despawn_recursive();
    }

    commands.spawn((
        Text::new(format!("Game over\nScore: {}\nPress Enter to restart.", score.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            width: Val::Vw(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        EndScreenElement
    ));
}

pub fn handle_restart_game(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    end_entities: Query<Entity, With<EndScreenElement>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        for end_entity in &end_entities {
            commands.entity(end_entity).despawn();
        }
        commands.set_state(GameState::InGame);
    }
}