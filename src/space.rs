use crate::*;
pub struct ModPlugin;
#[derive(Resource, Default, Debug)]
pub struct ScreenState {
    pub scale: f32,
    pub size: Vec2,
}

impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenState>();
    }
}

// 背景の碁盤目状のパネルと、カメラを生成
pub fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..PANEL_X_COUNT {
        for y in 0..PANEL_Y_COUNT {
            commands.spawn(block(
                x - (PANEL_X_COUNT - 1) / 2,
                y - (PANEL_Y_COUNT - 1) / 2,
            ));
        }
    }
}

fn block(x: isize, y: isize) -> SpriteBundle {
    return SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(PANEL_SIZE - 2.0, PANEL_SIZE - 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            x as f32 * PANEL_SIZE,
            y as f32 * PANEL_SIZE,
            0.,
        )),
        ..default()
    };
}

// positionをtranslationに変換
pub fn position_system(
    mut transform_query: Query<(&Position, &mut Transform)>,
    mut visibility_query: Query<(&Position, &mut Visibility)>,
) {
    for (position, mut transform) in transform_query.iter_mut() {
        transform.translation.y = position.y;
        transform.translation.x = position.x;
        transform.translation.z = position.z;
    }
    for (position, mut visibility) in visibility_query.iter_mut() {
        if position.visible {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
pub fn zoom_system(
    mut camera_query: Query<&mut OrthographicProjection>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut screen_state: ResMut<ScreenState>,
) {
    let window = window_query.single();
    let size = Vec2 {
        x: window.width(),
        y: window.height(),
    };
    if size == screen_state.size {
        return;
    }

    let mut camera = camera_query.single_mut();
    let y_ratio = (BOARD_Y_SIZE + BOARD_MARGIN) / size.y;
    let x_ratio = (BOARD_X_SIZE + BOARD_MARGIN) / size.x;
    camera.scale = if y_ratio < x_ratio { x_ratio } else { y_ratio };

    screen_state.size = size;
    screen_state.scale = camera.scale;
}
