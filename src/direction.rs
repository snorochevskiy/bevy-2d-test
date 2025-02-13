use bevy_math::Vec3;

/// Defines possible object orientations (direction is looking at).
/// \|/
/// -O-
/// /|\
pub enum Direction {
    Bottom, BottomLeft, Left, TopLeft, Top, TopRight, Right, BottomRight
}

/// Calculates a direction given vector points to.
/// 
/// ```ascii
///                    ^
///                    |TOP
///             , - ~ ~|~ - ,
///          , '-------|-------'-
///        , |         |0.9     |  ,
///       ,  |  TOP    | TOP    |   ,
///      ,   |  LEFT   | RIGHT  |0.9 ,
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
        if y >= 0.9 {
            Direction::Top   
        } else if x >= 0.9 {
            Direction::Right
        } else if x <= -0.9 {
            Direction::Left
        } else if x > 0.0 {
            Direction::TopRight
        } else {
            Direction::TopLeft
        }
    } else {
        if y <= -0.9 {
            Direction::Bottom
        } else if x >= 0.9 {
            Direction::Right
        } else if x <= -0.9 {
            Direction::Left
        } else if x > 0.0 {
            Direction::BottomRight
        } else {
            Direction::BottomLeft
        }
    }
}
