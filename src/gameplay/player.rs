use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{animation::{AnimationByDirection, AnimationConfig}, coords::calc_mouse_world_coord, direction::direction_of_vector};

use super::{GRP_ENEMY, GRP_ENVIRONMENT, GRP_PLAYER, GRP_PLAYER_BULLET, CollidingObj, LevelComponents};

const TEXTURE_PLAYER: &str = "sprites/player.png";

const TEXTURE_BULLET: &str = "sprites/ball.png";

const SPEED_PLAYER: f32 = 100.0;

const SPEED_BULLET: f32 = 200.0;

const BULLET_LIFE_TIME: Duration = Duration::from_secs(1);

#[derive(Component)]
pub struct PlayerInfo {
    pub health: u32,
}

#[derive(Component)]
pub struct Bullet {
    elapsed: Timer,
}

#[derive(Resource)]
pub struct BulletSprite(pub Handle<Image>);

#[derive(Event)]
pub struct PlayerDamage(pub u32);

#[derive(Event)]
pub struct BulletCollided(pub Entity);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_texture: Handle<Image> = asset_server.load(TEXTURE_PLAYER);

    // Create player
    {
        let player_texture: Handle<Image> = player_texture;
        let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 8, 7, None, None);
        let player_atlas_layout = texture_atlas_layouts.add(player_layout);
        let idle_frames = AnimationByDirection {
            bottom: 0 .. 0,
            bottm_left: 1 .. 1,
            left: 1 .. 1,
            top_left: 2 .. 2,
            top: 5 .. 5,
            top_right: 4 .. 4,
            right: 3 .. 3,
            bottom_right: 3 .. 3,
        };
        let walk_frames = AnimationByDirection {
            bottom: 8 .. 15,
            bottm_left: 16 .. 23,
            left: 16 .. 23,
            top_left: 24 .. 31,
            top: 48 .. 55,
            top_right: 40 .. 47,
            right: 32 .. 39,
            bottom_right: 8 .. 15,
        };
        let start_animation = idle_frames.bottom.clone();
        let player_animation_config = AnimationConfig {
            idle: idle_frames,
            walk: walk_frames,
            dying: 0..0,
            current_frame_range: start_animation,
            fps: 10,
            elapsed_frame_timer: AnimationConfig::timer_from_fps(10),
        };

        commands.spawn((
            RigidBody::Dynamic,
            KinematicCharacterController::default(),
            ActiveEvents::COLLISION_EVENTS, // to receive event on colliding with enemy
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(7.0, 12.0),
            CollisionGroups::new(
                GRP_PLAYER,
                GRP_ENVIRONMENT | GRP_ENEMY,
            ),
            GravityScale(0.0),
            Dominance::group(100),
            Sprite {
                image: player_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            PlayerInfo { health: 100 },
            player_animation_config,
            Velocity::zero(),
            LevelComponents,
            CollidingObj::Player,
        ));
    }

    let bullet_texture: Handle<Image> = asset_server.load(TEXTURE_BULLET);
    commands.insert_resource(BulletSprite(bullet_texture));
}

pub fn execute_player_behavior(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    textures: Res<BulletSprite>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), Without<AnimationConfig>>,
    mut player_query: Query<(&mut AnimationConfig, &mut Velocity, &mut Transform), With<PlayerInfo>>,
) {
    let (mut animation, mut velocity, mut transform) = player_query.single_mut();

    let move_direction =
        if keys.pressed(KeyCode::KeyA) && keys.pressed(KeyCode::KeyS) {
            Vec3::new(-1.0, -1.0, 0.0).normalize()
        } else if keys.pressed(KeyCode::KeyA) && keys.pressed(KeyCode::KeyW) {
            Vec3::new(-1.0, 1.0, 0.0).normalize()
        } else if keys.pressed(KeyCode::KeyD) && keys.pressed(KeyCode::KeyS) {
            Vec3::new(1.0, -1.0, 0.0).normalize()
        } else if keys.pressed(KeyCode::KeyD) && keys.pressed(KeyCode::KeyW) {
            Vec3::new(1.0, 1.0, 0.0).normalize()
        } else if keys.pressed(KeyCode::KeyA) {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if keys.pressed(KeyCode::KeyW) {
            Vec3::new(0.0, 1.0, 0.0)
        } else if keys.pressed(KeyCode::KeyD) {
            Vec3::new(1.0, 0.0, 0.0)
        } else if keys.pressed(KeyCode::KeyS) {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::ZERO
        };

    let (camera, mut camera_transform, camera_global_transform) = camera_query.single_mut();
    if let Some(cursor_position) = window_query.single().cursor_position() {
        if let Some(coord) = calc_mouse_world_coord(cursor_position, &camera, &camera_global_transform) {
            let player_orientation = (coord - transform.translation).normalize();
            let look_direction = direction_of_vector(player_orientation);
            let frames_range = if move_direction != Vec3::ZERO {
                animation.walk.for_direction(look_direction)
            } else {
                animation.idle.for_direction(look_direction)
            };

            if animation.current_frame_range != frames_range {
                animation.current_frame_range = frames_range;
            }

            if mouse.just_pressed(MouseButton::Middle) {
                println!("Clicked coordinates: {coord:?}");
            }

            if mouse.just_pressed(MouseButton::Left) {
                commands.spawn((
                    Sprite {
                        image: textures.0.clone(),
                        ..default()
                    },
                    transform.clone(),
                    Bullet {
                        elapsed: Timer::new(BULLET_LIFE_TIME, TimerMode::Once),
                    },
                    RigidBody::Dynamic,
                    GravityScale(0.0),
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(2.0),
                    CollisionGroups::new(
                        GRP_PLAYER_BULLET,
                        GRP_ENVIRONMENT | GRP_ENEMY,
                    ),
                    ActiveEvents::COLLISION_EVENTS,
                    Velocity::linear(player_orientation.xy() * SPEED_BULLET),
                    LevelComponents,
                    CollidingObj::Bullet,
                ));
            }
        };
    }

    velocity.linvel = (move_direction * SPEED_PLAYER).xy();

    transform.translation.z = -(transform.translation.y * 0.01);

    camera_transform.translation.x = transform.translation.x;
    camera_transform.translation.y = transform.translation.y;
}

pub fn execute_bullets_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
) {
    for (bullet_entity, mut bullet_info) in &mut bullet_query {
        bullet_info.elapsed.tick(time.delta());
        if bullet_info.elapsed.just_finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn on_player_damaged(
    mut events: EventReader<PlayerDamage>,
    mut player_query: Single<&mut PlayerInfo>,
) {
    for PlayerDamage(dmg) in events.read() {
        player_query.health = player_query.health.saturating_sub(*dmg);
    }
}

pub fn on_bullet_collided(
    mut commands: Commands,
    mut events: EventReader<BulletCollided>,
) {
    for BulletCollided(bullet_entity) in events.read() {
        commands.entity(*bullet_entity).despawn();
    }
}