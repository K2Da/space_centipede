use crate::*;

pub struct ModPlugin;

// ユーザーの入力をリソースに設定する
impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CursorState>()
            .add_system(read_input_events_system.system());
    }
}

#[derive(Default, Debug)]
pub struct CursorState {
    pub screen_position: Vec2,
    pub position: Position,
    pub left_pressed: bool,
}

// bevyのResから、情報を読み取り、CursorStateを更新する
fn read_input_events_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut cursor_state: ResMut<CursorState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_moved_reader: Local<EventReader<CursorMoved>>,
) {
    // cursorは左下が0, 0、Vec2は真ん中が0, 0
    for event in cursor_moved_reader.iter(&cursor_moved_events) {
        cursor_state.screen_position = event.position;
    }

    // マウスの左ボタン状態
    cursor_state.left_pressed = mouse_input.pressed(MouseButton::Left);

    let window = windows.get_primary().unwrap();
    cursor_state.position.x = cursor_state.screen_position.x - window.width() / 2.0;
    cursor_state.position.y = cursor_state.screen_position.y - window.height() / 2.0;
}
