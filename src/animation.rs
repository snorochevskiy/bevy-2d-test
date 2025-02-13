use std::{ops::Range, time::Duration};

use bevy::prelude::*;

use crate::direction::Direction;

/// For an animations specifies what frames intervals correspond to each of directions.
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

/// Generalized animation config for any moving character/enemy.
/// We assume that each such object should have idel, walk and dying animation.
#[derive(Component)]
pub struct AnimationConfig {
    pub idle: AnimationByDirection,
    pub walk: AnimationByDirection,
    pub dying: Range<usize>,
    pub current_frame_range: Range<usize>,
    pub fps: u8,
    pub elapsed_frame_timer: Timer, //move this to struct per instance, everything else can be shared
}

impl AnimationConfig {
    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
    pub fn frame_show_time(&self) -> Duration {
        Duration::from_secs_f32(1.0 / (self.fps as f32))
    }
}

/// Plays sprite anumation for all object that have thier animations defined via [[AnimationConfig]].
pub fn play_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        // We track how much time passed since previous frame.
        config.elapsed_frame_timer.tick(time.delta());

        // Then we check if it is time to play next animation frame.
        // If not, we leave current frame.
        // Otherwise we switch to the next frame and restart the frame timer.
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