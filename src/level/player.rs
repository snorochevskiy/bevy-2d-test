use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{animation::{AnimationByDirection, AnimationConfig}, coords::calc_mouse_world_coord, direction::direction_of_vector};

use super::{enemy::EnemyState, LevelComponents};

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
    direction: Vec3,
    elapsed: Timer,
}

#[derive(Resource)]
pub struct BulletSprite(pub Handle<Image>);

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
            Collider::cuboid(8.0, 12.0),
            GravityScale(0.0),
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

    // TODO: move these key checks to separate system.
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

            if mouse.just_pressed(MouseButton::Right) {
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
                        direction: player_orientation,
                        elapsed: Timer::new(BULLET_LIFE_TIME, TimerMode::Once),
                    },
                    LevelComponents,
                ));
            }
        };
    }

    velocity.linvel = (move_direction * SPEED_PLAYER * 2.0).xy();

    transform.translation.z = -(transform.translation.y * 0.01);

    camera_transform.translation.x = transform.translation.x;
    camera_transform.translation.y = transform.translation.y;
}

pub fn execute_bullets_move(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Transform, &mut Bullet)>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Velocity, &mut AnimationConfig, &mut EnemyState), Without<Bullet>>,
) {
    for (bullet_entity, mut bullet_transform, mut bullet_info) in &mut bullet_query {
        bullet_info.elapsed.tick(time.delta());
        if bullet_info.elapsed.just_finished() {
            commands.entity(bullet_entity).despawn();
        } else {
            bullet_transform.translation += bullet_info.direction * time.delta_secs() * SPEED_BULLET;

            for (enemy_entity, enemy_transform, mut enemy_velocity, mut enemy_anim_config, mut enemy_info) in &mut enemy_query {
                // Just ot demonstrate that we can detect bullet collion with an enemy just by checking coordinates, without physics

                let enemy_hit =
                    bullet_transform.translation.x >= enemy_transform.translation.x - 12.0 && 
                    bullet_transform.translation.x <= enemy_transform.translation.x + 12.0 && 
                    bullet_transform.translation.y >= enemy_transform.translation.y - 12.0 && 
                    bullet_transform.translation.y <= enemy_transform.translation.y + 12.0 &&
                    enemy_info.is_alive();

                if  enemy_hit {
                        commands.entity(bullet_entity).despawn();
                        let dying_duration = enemy_anim_config.frame_show_time() * enemy_anim_config.dying.len() as u32;
                        *enemy_info = EnemyState::Dying(Timer::new(dying_duration, TimerMode::Once));
                        enemy_anim_config.current_frame_range = enemy_anim_config.dying.clone();
                        commands.entity(enemy_entity).remove::<Collider>();
                        enemy_velocity.linvel = Vec2::ZERO;
                        break;
                }
            }
        }
    }
}