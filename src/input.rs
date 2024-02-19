use crate::space::ScreenState;
use crate::*;

pub struct ModPlugin;

// ユーザーの入力をリソースに設定する
impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorState>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct CursorState {
    pub screen_position: Vec2,
    pub position: Position,
    pub left_pressed: bool,
}

// bevyのResから、情報を読み取り、CursorStateを更新する
pub fn read_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    screen_state: Res<ScreenState>,
    mut cursor_state: ResMut<CursorState>,
    mut cursor_reader: EventReader<CursorMoved>,
    mut tap_events: ResMut<Events<event::Tap>>,
) {
    // cursorは左下が0, 0、Vec2は真ん中が0, 0
    for event in cursor_reader.read() {
        cursor_state.screen_position = event.position;
    }

    // ユーザーのタップ(マウスの左ボタン)
    if !cursor_state.left_pressed && mouse_input.pressed(MouseButton::Left) {
        tap_events.send(event::Tap {});
    }
    cursor_state.left_pressed = mouse_input.pressed(MouseButton::Left);

    cursor_state.position.x =
        (cursor_state.screen_position.x - screen_state.size.x / 2.0) * screen_state.scale;
    cursor_state.position.y =
        (-cursor_state.screen_position.y + screen_state.size.y / 2.0) * screen_state.scale;
}
