use std::{ops::Range, time::Duration};

use bevy::{ecs::component::Component, time::{Timer, TimerMode}};

use crate::direction::Direction;

#[derive(Debug, Default, Clone)]
pub struct AnimationByDirection {
    pub bottom: Range<usize>,
    pub bottm_left: Range<usize>,
    pub left: Range<usize>,
    pub top_left: Range<usize>,
    pub top: Range<usize>,
    pub top_right: Range<usize>,
    pub right: Range<usize>,
    pub bottom_right: Range<usize>,
}

impl AnimationByDirection {
    pub fn for_direction(&self, d: Direction) -> Range<usize> {
        match d {
            Direction::Bottom => self.bottom.clone(),
            Direction::BottomLeft => self.bottm_left.clone(),
            Direction::Left => self.left.clone(),
            Direction::TopLeft => self.top_left.clone(),
            Direction::Top => self.top.clone(),
            Direction::TopRight => self.top_right.clone(),
            Direction::Right => self.right.clone(),
            Direction::BottomRight => self.bottom_right.clone(),
        }
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    pub idle: AnimationByDirection,
    pub walk: AnimationByDirection,
    pub current_frame_range: Range<usize>,
    pub fps: u8,
    pub elapsed_frame_timer: Timer, //move this to struct per instance, everything else can be shared
}

impl AnimationConfig {
    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}