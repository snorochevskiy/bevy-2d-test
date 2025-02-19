use bevy_math::Vec3;

/// Defines possible object orientations (direction is looking at).
/// \|/
/// -O-
/// /|\
pub enum Direction {
    Bottom, BottomLeft, Left, TopLeft, Top, TopRight, Right, BottomRight
}

const ANGLE_THRESHOLD: f32 = 0.85;

/// Calculates a direction given vector points to.
/// 
/// ```ascii
///                    ^
///                    |TOP
///             , - ~ ~|~ - ,
///          , '-------|-------'-
///        , |         |0.85    |  ,
///       ,  |  TOP    | TOP    |   ,
///      ,   |  LEFT   | RIGHT  |0.85,
///     -----|---------+--------|------>
///  LEFT,   | BOTTOM  | BOTTOM |    ,RIGHT
///       ,  |  LEFT   | RIGHT  |   ,
///        , |         |        |  ,
///          ,------------------, '
///            ' - , _ | _ ,  '
///                    BOTTOM
/// ```
pub fn direction_of_vector(Vec3 {x, y, z: _} : Vec3) -> Direction {
    if y > 0.0 {
        if y >= ANGLE_THRESHOLD {
            Direction::Top   
        } else if x >= ANGLE_THRESHOLD {
            Direction::Right
        } else if x <= -ANGLE_THRESHOLD {
            Direction::Left
        } else if x > 0.0 {
            Direction::TopRight
        } else {
            Direction::TopLeft
        }
    } else {
        if y <= -ANGLE_THRESHOLD {
            Direction::Bottom
        } else if x >= ANGLE_THRESHOLD {
            Direction::Right
        } else if x <= -ANGLE_THRESHOLD {
            Direction::Left
        } else if x > 0.0 {
            Direction::BottomRight
        } else {
            Direction::BottomLeft
        }
    }
}
