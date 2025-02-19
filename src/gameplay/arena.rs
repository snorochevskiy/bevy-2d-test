use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{GRP_ENEMY, GRP_ENVIRONMENT, GRP_PLAYER, GRP_PLAYER_BULLET, CollidingObj, LevelComponents};

const TEXTURE_ARENA: &str = "sprites/arena.png";

pub fn setup_arena(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let arena_texture: Handle<Image> = asset_server.load(TEXTURE_ARENA);

    // Create arena background
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
        let collision_group = CollisionGroups::new(
            GRP_ENVIRONMENT,
            GRP_ENVIRONMENT | GRP_PLAYER | GRP_ENEMY | GRP_PLAYER_BULLET,
        );
        commands.spawn(( // top border
            RigidBody::Fixed,
            Collider::cuboid(370.0, 10.0),
            collision_group,
            Transform::from_translation(Vec3::new(0.0, 330.0, 0.0)),
            LevelComponents,
            CollidingObj::Environment,
        ));
        commands.spawn(( // bottom border
            RigidBody::Fixed,
            Collider::cuboid(370.0, 10.0),
            collision_group,
            Transform::from_translation(Vec3::new(0.0, -390.0, 0.0)),
            LevelComponents,
            CollidingObj::Environment,
        ));
        commands.spawn(( // left border
            RigidBody::Fixed,
            Collider::cuboid(10.0, 380.0),
            collision_group,
            Transform::from_translation(Vec3::new(-380.0, 0.0, 0.0)),
            LevelComponents,
            CollidingObj::Environment,
        ));
        commands.spawn(( // right border
            RigidBody::Fixed,
            Collider::cuboid(10.0, 380.0),
            collision_group,
            Transform::from_translation(Vec3::new(380.0, 0.0, 0.0)),
            LevelComponents,
            CollidingObj::Environment,
        ));
    }
}