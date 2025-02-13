use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct StartScreenElement;

pub fn setup_start_screen(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("Use WASD to move and Mouse to shoot.\nUse mouse wheel (on desktop) or Z ans X for camera zoom.\nPress Enter to start."),
        TextLayout::new_with_justify(JustifyText::Center),
        //Outline::new(Val::Px(5.), Val::Px(5.), BLACK.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            width: Val::Vw(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        StartScreenElement
    ));
}

pub fn handle_start_game(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<StartScreenElement>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
        commands.set_state(AppState::InGame);
    }
}