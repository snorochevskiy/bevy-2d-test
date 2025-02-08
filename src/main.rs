mod direction;
mod coords;
mod animation;

use std::time::Duration;

use animation::{AnimationByDirection, AnimationConfig};
use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};
use bevy_framepace::{FramepaceSettings, Limiter};
use coords::calc_mouse_world_coord;
use direction::direction_of_vector;

const TEXTURE_BULLET: &str = "sprites/ball.png";
const TEXTURE_ARENA: &str = "sprites/arena.png";
const TEXTURE_PLAYER: &str = "sprites/player.png";
const TEXTURE_SLIME: &str = "sprites/slime.png";

const SPEED_PLAYER: f32 = 100.0;
const SPEED_SLIME: f32 = 50.0;
const SPEED_BULLET: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, init_configs_system)
        .add_systems(Startup, setup)
        .add_systems(Update, execute_player_movement)
        .add_systems(Update, execute_sprite_animations)
        .add_systems(Update, execute_enemy_movement)
        .add_systems(Update, execute_bullets_move)
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, scroll_events)
        .run();
}

fn init_configs_system(
    mut fps: ResMut<FramepaceSettings>,
) {
    fps.limiter = Limiter::from_framerate(30.0);
}


#[derive(Component)]
struct Bullet {
    direction: Vec3,
    elapsed: Timer,
}

fn execute_player_movement(
    mut commands: Commands,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    textures: Res<SpawnConfigs>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), Without<AnimationConfig>>,
    mut player_query: Query<(&mut AnimationConfig, &mut Transform), With<PlayerMarker>>,
) {
    let (mut animation, mut transform) = player_query.single_mut();

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
                println!("{coord:?}");
            }

            if mouse.just_pressed(MouseButton::Left) {
                commands.spawn((
                    Sprite {
                        image: textures.bullet.clone(),
                        ..default()
                    },
                    transform.clone(),
                    Bullet {
                        direction: player_orientation,
                        elapsed: Timer::new(Duration::from_secs(1), TimerMode::Once),
                    }
                ));
            }
        };
    }
    
    transform.translation += move_direction * time.delta_secs() * SPEED_PLAYER;

    if transform.translation.x < -2170.0 {
        transform.translation.x = -2170.0;
    }
    if transform.translation.x > 2170.0 {
        transform.translation.x = 2170.0;
    }
    if transform.translation.y > 1960.0 {
        transform.translation.y = 1960.0;
    }
    if transform.translation.y < -2220.0 {
        transform.translation.y = -2220.0;
    }

    transform.translation.z = -(transform.translation.y * 0.01);

    camera_transform.translation.x = transform.translation.x;
    camera_transform.translation.y = transform.translation.y;
}

fn scroll_events(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let (_, mut camera_transform) = camera_query.single_mut();
                let factor = ev.y * -0.02;
                let current_scale = camera_transform.scale.x;
                    let mut new_scale = ((current_scale + factor) * 100.0).floor() / 100.0;
                    if new_scale < 0.12 {
                        new_scale = 0.12;
                    }
                    camera_transform.scale = Vec3::new(new_scale, new_scale, new_scale);

                println!("{:?}", camera_transform.scale);
            }
            MouseScrollUnit::Pixel => (),
        }
    }
}

fn execute_bullets_move(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Transform, &mut Bullet)>,
    enemy_query: Query<(Entity, &mut Transform, &EnemyMarker), Without<Bullet>>,
) {
    for (bullet_entity, mut bullet_transform,  mut bullet_info) in &mut bullet_query {
        bullet_info.elapsed.tick(time.delta());
        if bullet_info.elapsed.just_finished() {
            commands.entity(bullet_entity).despawn();
        } else {
            bullet_transform.translation += bullet_info.direction * time.delta_secs() * SPEED_BULLET;

            for (enemy_entity, enemy_transform, _) in &enemy_query {
                if bullet_transform.translation.x >= enemy_transform.translation.x - 12.0 && 
                    bullet_transform.translation.x <= enemy_transform.translation.x + 12.0  && 
                    bullet_transform.translation.y >= enemy_transform.translation.y - 12.0 && 
                    bullet_transform.translation.y <= enemy_transform.translation.y + 12.0  {
                        commands.entity(bullet_entity).despawn();
                        commands.entity(enemy_entity).despawn();
                        break;
                }
            }
        }
    }
}

fn execute_enemy_movement(
    time: Res<Time>,
    player_query: Query<(&Transform, &PlayerMarker)>,
    mut enemy_query: Query<(&mut Transform, &EnemyMarker), Without<PlayerMarker>>,
) {
    let (player_transform, _) = player_query.single();
    
    for (mut enemy_transfrom, _) in &mut enemy_query {
        let move_vector = (player_transform.translation - enemy_transfrom.translation).normalize();
        enemy_transfrom.translation += move_vector * time.delta_secs() * SPEED_SLIME;
        enemy_transfrom.translation.z = -(enemy_transfrom.translation.y * 0.01);
    }
}

fn execute_sprite_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        config.elapsed_frame_timer.tick(time.delta());

        if config.elapsed_frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if !config.current_frame_range.contains(&atlas.index) {
                    atlas.index = config.current_frame_range.start;
                } else if atlas.index == config.current_frame_range.end {
                    atlas.index = config.current_frame_range.start;
                } else {
                    atlas.index += 1;
                }
                config.elapsed_frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct EnemyMarker;

#[derive(Default, Clone, Resource)]
pub struct SpawnConfigs {
    bullet: Handle<Image>,
    slime: EnemySpriteSpawnConfig,
}

#[derive(Default, Clone)]
pub struct EnemySpriteSpawnConfig {
    texture: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    walk_frames: AnimationByDirection,
    idle_frames: AnimationByDirection,
}

#[derive(Default, Clone, Resource)]
pub struct EnemySpawnerState {
    locations: Vec<Vec3>,
    timer: Timer,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let ball_texture: Handle<Image> = asset_server.load(TEXTURE_BULLET);

    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::ONE * 0.2),
    ));

    // Background
    {
        let arena: Handle<Image> = asset_server.load(TEXTURE_ARENA);
        commands.spawn((
            Sprite {
                image: arena,
                .. default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
        ));
    }

    // Player
    {
        let player_texture: Handle<Image> = asset_server.load(TEXTURE_PLAYER);
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
            current_frame_range: start_animation,
            fps: 10,
            elapsed_frame_timer: AnimationConfig::timer_from_fps(10),
        };

        commands.spawn((
            Sprite {
                image: player_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_atlas_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(-20.0, 0.0, 0.0)),
            PlayerMarker,
            player_animation_config,
        ));
    }

    // Slime
    commands.insert_resource(EnemySpawnerState {
        locations: vec![
            Vec3::new(0.0, -15.0, 0.0),
            Vec3::new(-345.0, 295.0, 0.0),
            Vec3::new(345.0, 295.0, 0.0),
            Vec3::new(-345.0, -360.0, 0.0),
            Vec3::new(345.0, -360.0, 0.0),
        ],
        timer: Timer::from_seconds(0.3, TimerMode::Repeating)
    });

    let slime_sprite_spawn_config = {
        let slime_texture: Handle<Image> = asset_server.load(TEXTURE_SLIME);
        let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 4, None, None);
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
        }
    };

    commands.insert_resource(SpawnConfigs {
        bullet: ball_texture,
        slime: slime_sprite_spawn_config,
    });

}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    spawn_configs: Res<SpawnConfigs>,
    mut enemy_spawner_state: ResMut<EnemySpawnerState>,
) {
    enemy_spawner_state.timer.tick(time.delta());

    if enemy_spawner_state.timer.just_finished() {
        let location_index: usize = (rand::random::<usize>()) % enemy_spawner_state.locations.len();

        let start_animation = spawn_configs.slime.walk_frames.bottom.clone();
        let start_frame = start_animation.start;
        let slime_animation_config = AnimationConfig {
            idle: spawn_configs.slime.idle_frames.clone(),
            walk: spawn_configs.slime.walk_frames.clone(),
            current_frame_range: start_animation,
            fps: 10,
            elapsed_frame_timer: AnimationConfig::timer_from_fps(10),
        };
    
        commands.spawn((
            Sprite {
                image: spawn_configs.slime.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: spawn_configs.slime.atlas.clone(),
                    index: start_frame,
                }),
                ..default()
            },
            Transform::from_translation(enemy_spawner_state.locations[location_index]),
            EnemyMarker,
            slime_animation_config,
        ));
    }
}

