pub mod arena;
pub mod player;
pub mod enemy;

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct LevelComponents;