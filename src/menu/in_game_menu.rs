use bevy::{color::palettes::css::{RED, YELLOW}, prelude::*};

use crate::gameplay::{player::PlayerInfo, GameScore, LevelComponents};

#[derive(Component)]
pub struct HealthBar(u32);

#[derive(Component)]
pub struct ScoreBar(u32);

pub fn update_game_ui(
    player_query: Query<&PlayerInfo>,
    score: Res<GameScore>,
    mut health_query: Query<(&mut Text, &mut HealthBar), Without<ScoreBar>>,
    mut score_query: Query<(&mut Text, &mut ScoreBar), Without<HealthBar>>,
) {
    for (mut text, mut health_bar) in &mut health_query {
        for player_info in &player_query {
            if health_bar.0 != player_info.health {
                health_bar.0 = player_info.health;
                text.0 = health_text(health_bar.0);
            }
        }
    }

    for (mut text, mut score_bar) in &mut score_query {
        if score_bar.0 != score.0 {
            score_bar.0 = score.0;
            text.0 = score_text(score_bar.0);
        }
    }
}

pub fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameScore(0));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.),
            ..default()
        },
        LevelComponents,
    )).with_children(|builder| {
        builder
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(health_text(100)),
                TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: 48.0,
                    ..default()
                },
                TextColor(RED.into()),
                HealthBar(100),
                LevelComponents,
            ));
        });


        builder
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(score_text(0)),
                TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: 48.0,
                    ..default()
                },
                TextColor(YELLOW.into()),
                ScoreBar(0),
                LevelComponents,
            ));
        });
    });
}

fn health_text(points: u32) -> String {
    format!("♥ {points}")
}

fn score_text(points: u32) -> String {
    format!("★ {points}")
}