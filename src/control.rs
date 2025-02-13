use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn init_cursor(
    mut commands: Commands,
    wnd_entity: Single<Entity, With<Window>>,
    asset_server: Res<AssetServer>,
) {
    let cursor_icon: bevy::winit::cursor::CursorIcon = bevy::winit::cursor::CustomCursor::Image {
        handle: asset_server.load("icon/aim.png"),
        hotspot: (16, 16),
    }.into();

    commands
        .entity(*wnd_entity)
        .insert(cursor_icon);

}

pub fn handle_camera_zoom(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut evr_scroll: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let zoom = |camera_transform: &mut Transform, factor: f32| {
        let current_scale = camera_transform.scale.x;
        let mut new_scale = ((current_scale + factor) * 100.0).floor() / 100.0;
        if new_scale < 0.12 {
            new_scale = 0.12;
        }
        camera_transform.scale = Vec3::new(new_scale, new_scale, new_scale);
    };

    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let (_, mut camera_transform) = camera_query.single_mut();
                let factor = ev.y * -0.02;
                zoom(&mut camera_transform, factor);

            }
            MouseScrollUnit::Pixel => (),
        }
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        let (_, mut camera_transform) = camera_query.single_mut();
        zoom(&mut camera_transform, 0.02);
    }
    if keys.just_pressed(KeyCode::KeyX) {
        let (_, mut camera_transform) = camera_query.single_mut();
        zoom(&mut camera_transform, -0.02);
    }
}