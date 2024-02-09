use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Head {}

#[derive(Component)]
pub struct CenterMarker {}

#[derive(Component)]
pub struct Chain {}

pub fn setup_system(mut commands: Commands, handles: Res<asset::Handles>) {
    commands
        .spawn(handles.center_marker_mesh_bundle())
        .insert(CenterMarker {})
        .insert(Position::default(false, MARKER_Z));
    commands
        .spawn(handles.chain_bundle())
        .insert(Chain {})
        .insert(Position::default(true, MARKER_Z - 1.0));
}

pub fn select_movement_system(
    mut centipede_container: ResMut<CentipedeContainer>,
    cursor_state: Res<input::CursorState>,
    mut marker_query: Query<&mut Position, With<CenterMarker>>,
    mut chain_query: Query<
        (&mut Position, &mut Sprite, &mut Transform),
        (With<Chain>, Without<CenterMarker>),
    >,
    head_query: Query<&Position, (With<Head>, Without<CenterMarker>, Without<Chain>)>,
    mut tap_reader: EventReader<event::Tap>,
) -> Option<()> {
    let centipede = centipede_container.alive_mut()?;
    let head_position = head_query.get(centipede.head_entity).ok()?;
    let mut marker_position = marker_query.get_single_mut().ok()?;

    if tap_reader.read().count() % 2 == 1 {
        if matches!(centipede.movement, Movement::Circular(_)) {
            centipede.movement = Movement::Linear(centipede.last_move);
        } else {
            let vec = Vec2 {
                y: -cursor_state.position.x + head_position.x,
                x: cursor_state.position.y - head_position.y,
            };
            let inner_product = vec.x * centipede.last_move.x + vec.y * centipede.last_move.y;
            centipede.movement = Movement::Circular(CircularMove {
                center: cursor_state.position.clone(),
                clockwise: inner_product < 0.0,
            });
            marker_position.x = cursor_state.position.x;
            marker_position.y = cursor_state.position.y;
        }
    }

    let circular = matches!(centipede.movement, Movement::Circular(_));

    marker_position.visible = circular;
    let (mut chain_position, mut chain_sprite, mut chain_transform) =
        chain_query.get_single_mut().ok()?;
    chain_position.visible = circular;
    if circular {
        chain_position.x = (head_position.x + marker_position.x) / 2.0f32;
        chain_position.y = (head_position.y + marker_position.y) / 2.0f32;
        chain_sprite.custom_size = Some(Vec2::new(head_position.distance(&marker_position), 2.0));
        chain_transform.rotation = chain_position.head_to(*head_position);
    }
    None
}

pub fn move_head_system(
    mut centipede_container: ResMut<CentipedeContainer>,
    time: Res<Time>,
    mut head_query: Query<&mut Position, With<Head>>,
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;
    let mut position = head_query.get_mut(centipede.head_entity).ok()?;

    // 壁の外にいたら無条件に跳ね返す
    reverse_head_move(&mut centipede, &mut position);

    let distance = centipede.speed * time.delta_seconds();
    let last_position = position.clone();

    match centipede.movement {
        Movement::Circular(CircularMove { center, clockwise }) => {
            let radius: f32 = position.distance(&center);
            let radian: f32 = (position.x - center.x).atan2(position.y - center.y)
                + distance / radius * if clockwise { 1.0 } else { -1.0 };

            position.x = center.x + radian.sin() * radius;
            position.y = center.y + radian.cos() * radius;
            centipede.speed += time.delta_seconds() * SPEED_UP;
        }
        Movement::Linear(direction) => {
            if direction != (Vec2 { x: 0.0, y: 0.0 }) {
                position.move_to_with_distance(direction, distance);
            }
        }
    }

    centipede.last_move = Vec2 {
        x: position.x - last_position.x,
        y: position.y - last_position.y,
    };

    centipede.position_history.push(position.clone());
    None
}

fn reverse_head_move(centipede: &mut Alive, position: &mut Mut<Position>) {
    let (out_x, out_y) = (
        position.x > BOARD_X_BORDER && centipede.last_move.x > 0.0
            || position.x < -BOARD_X_BORDER && centipede.last_move.x < 0.0,
        position.y > BOARD_Y_BORDER && centipede.last_move.y > 0.0
            || position.y < -BOARD_Y_BORDER && centipede.last_move.y < 0.0,
    );

    if out_x || out_y {
        let Vec2 { x, y } = centipede.last_move;
        centipede.movement = Movement::Linear(Vec2 {
            x: if out_x { -x } else { x },
            y: if out_y { -y } else { y },
        })
    }
}

pub fn on_game_start(
    mut commands: Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    mut start_reader: EventReader<event::GameStart>,
    handles: Res<asset::Handles>,
) {
    for _ in start_reader.read() {
        centipede_container.centipede = Centipede::Alive(Alive::default(
            commands
                .spawn(handles.head_mesh_bundle())
                .insert(Head {})
                .insert(Position::default(true, CENTIPEDE_Z))
                .id(),
        ));
    }
}

pub fn on_game_over(mut commands: Commands, mut over_reader: EventReader<event::GameOver>) {
    for event in over_reader.read() {
        commands.entity(event.head_entity).despawn();
    }
}
