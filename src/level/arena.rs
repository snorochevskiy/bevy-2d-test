use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use super::LevelComponents;

const TEXTURE_ARENA: &str = "sprites/arena.png";

pub fn setup_arena(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let arena_texture: Handle<Image> = asset_server.load(TEXTURE_ARENA);

    // Create level background
    {
        commands.spawn((
            Sprite {
                image: arena_texture,
                .. default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
            LevelComponents,
        ));
    }

    // Scene colliders
    {
        commands.spawn(( // top
            RigidBody::Fixed,
            Collider::cuboid(370.0, 10.0),
            Transform::from_translation(Vec3::new(0.0, 330.0, 0.0)),
            LevelComponents,
        ));
        commands.spawn(( // bottom
            RigidBody::Fixed,
            Collider::cuboid(370.0, 10.0),
            Transform::from_translation(Vec3::new(0.0, -390.0, 0.0)),
            LevelComponents,
        ));
        commands.spawn(( // left
            RigidBody::Fixed,
            Collider::cuboid(10.0, 380.0),
            Transform::from_translation(Vec3::new(-380.0, 0.0, 0.0)),
            LevelComponents,
        ));
        commands.spawn(( // right
            RigidBody::Fixed,
            Collider::cuboid(10.0, 380.0),
            Transform::from_translation(Vec3::new(380.0, 0.0, 0.0)),
            LevelComponents,
        ));
    }
}