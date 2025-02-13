use bevy::{color::palettes::css::RED, prelude::*};

use crate::level::{player::PlayerInfo, LevelComponents};

#[derive(Component)]
pub struct HealthBar(u32);

pub fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text::new("♥ 100"),
        TextFont {
            font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
            font_size: 48.0,
            ..Default::default()
        },
        TextColor(RED.into()),
        //Outline::new(Val::Px(5.), Val::Px(5.), BLACK.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        HealthBar(100),
        LevelComponents,
    ));
}

pub fn update_game_ui(
    player_query: Query<&PlayerInfo>,
    mut health_query: Query<(&mut Text, &mut HealthBar)>,
) {
    for (mut text, mut health_bar) in &mut health_query {
        for player_info in &player_query {
            if health_bar.0 != player_info.health {
                health_bar.0 = player_info.health;
                text.0 = format!("♥ {}", health_bar.0);
            }
        }
    }
}