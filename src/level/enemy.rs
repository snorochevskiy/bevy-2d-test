use std::{collections::HashSet, ops::Range, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::{AnimationByDirection, AnimationConfig}, AppState};

use super::{player::PlayerInfo, LevelComponents};

const TEXTURE_SLIME: &str = "sprites/slime.png";

const SPEED_SLIME: f32 = 50.0;

const DAMAGE_SLIME: u32 = 10;

const ENEMY_SPAWN_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Component, PartialEq, Eq)]
pub enum EnemyState {
    Alive, Dying(Timer)
}

impl EnemyState {
    pub fn is_alive(&self) -> bool {
        match self {
            EnemyState::Alive => true,
            _ => false,
        }
    }
}

#[derive(Default, Clone, Resource)]
pub struct EnemySpriteSpawnConfig {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub walk_frames: AnimationByDirection,
    pub idle_frames: AnimationByDirection,
    pub dying_frames: Range<usize>,
}

#[derive(Default, Clone, Resource)]
pub struct EnemySpawner {
    pub locations: Vec<Vec3>,
    pub timer: Timer,
}

pub fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let slime_texture: Handle<Image> = asset_server.load(TEXTURE_SLIME);

    // List of locations where enemies can be spawned and spawning timer
    commands.insert_resource(EnemySpawner {
        locations: vec![
            Vec3::new(0.0, -15.0, 0.0),
            Vec3::new(-345.0, 295.0, 0.0),
            Vec3::new(345.0, 295.0, 0.0),
            Vec3::new(-345.0, -360.0, 0.0),
            Vec3::new(345.0, -360.0, 0.0),
        ],
        timer: Timer::new(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating)
    });

    let slime_sprite_spawn_config = {
        let slime_texture: Handle<Image> = slime_texture;
        let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 5, None, None);
        let slime_atlas_layout = texture_atlas_layouts.add(slime_layout);
        let walk_frames = AnimationByDirection {
            bottom: 0 .. 7,
            bottm_left: 0 .. 7,
            left: 16 .. 23,
            top_left: 8 .. 15,
            top: 8 .. 15,
            top_right: 8 .. 15,
            right: 24 .. 31,
            bottom_right: 0 .. 7,
        };
        let idle_frames = walk_frames.clone();
        EnemySpriteSpawnConfig {
            texture: slime_texture,
            atlas: slime_atlas_layout,
            walk_frames,
            idle_frames,
            dying_frames: 35 .. 39,
        }
    };

    commands.insert_resource(slime_sprite_spawn_config);

}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    spawn_configs: Res<EnemySpriteSpawnConfig>,
    mut spawner: ResMut<EnemySpawner>,
) {
    spawner.timer.tick(time.delta());

    if spawner.timer.just_finished() {
        let location_index: usize = (rand::random::<usize>()) % spawner.locations.len();

        let start_animation = spawn_configs.walk_frames.bottom.clone();
        let start_frame = start_animation.start;
        let slime_animation_config = AnimationConfig {
            idle: spawn_configs.idle_frames.clone(),
            walk: spawn_configs.walk_frames.clone(),
            dying: spawn_configs.dying_frames.clone(),
            current_frame_range: start_animation,
            fps: 10,
            elapsed_frame_timer: AnimationConfig::timer_from_fps(10),
        };
    
        commands.spawn((
            RigidBody::Dynamic,
            GravityScale(0.0),
            Collider::ball(10.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            Sprite {
                image: spawn_configs.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: spawn_configs.atlas.clone(),
                    index: start_frame,
                }),
                ..default()
            },
            Transform::from_translation(spawner.locations[location_index]),
            EnemyState::Alive,
            slime_animation_config,
            LevelComponents,
        ));
    }
}

pub fn execute_enemy_behavior(
    mut commands: Commands,
    time: Res<Time>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &mut PlayerInfo)>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Velocity, &mut AnimationConfig, &mut EnemyState), Without<PlayerInfo>>,
) {
    let (player_entity, player_transform, mut player_info) = player_query.single_mut();

    // Collect IDs of enemies collided with the player
    let mut collided_enemies = HashSet::new();
    for collision_event in collision_events.read() {
        if let &CollisionEvent::Started(c1, c2 , _) = collision_event {
            if c1 == player_entity {
                collided_enemies.insert(c2);
            } else if c2 == player_entity {
                collided_enemies.insert(c1);
            };
        }
    }

    // Iterate through enemies
    for (enemy_entity, mut enemy_transfrom, mut enemy_velocity, mut enemy_anim_config, mut enemy_info) in &mut enemy_query {
        match enemy_info.as_mut() {
            EnemyState::Alive => {
                if collided_enemies.contains(&enemy_entity) {
                    player_info.health = player_info.health.saturating_sub(DAMAGE_SLIME);

                    if player_info.health == 0 {
                        commands.set_state(AppState::End); // TODO: move out?
                    }

                    let dying_duration = enemy_anim_config.frame_show_time() * enemy_anim_config.dying.len() as u32;
                    *enemy_info = EnemyState::Dying(Timer::new(dying_duration, TimerMode::Once));
                    enemy_anim_config.current_frame_range = enemy_anim_config.dying.clone();

                    commands.entity(enemy_entity).remove::<Collider>();
                    enemy_velocity.linvel = Vec2::ZERO;
                } else {
                    // Do the movement
                    let move_vector = (player_transform.translation - enemy_transfrom.translation).normalize();

                    // If perform enemy movement manually
                    //enemy_transfrom.translation += move_vector * time.delta_secs() * SPEED_SLIME;

                    // If perform enemy movement via Rapier physics
                    enemy_velocity.linvel = move_vector.xy() * SPEED_SLIME * 2.0;

                    // We update Z axis to implement correct sparites overlapping order
                    enemy_transfrom.translation.z = -(enemy_transfrom.translation.y * 0.01);
                }
            },
            EnemyState::Dying(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    commands.entity(enemy_entity).despawn();
                }
            },
        };
    }
}
