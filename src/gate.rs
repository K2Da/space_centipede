use crate::*;
use rand::prelude::random;

pub struct ModPlugin {}

impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GatesInfo>();
    }
}

#[derive(Resource, Default)]
pub struct GatesInfo {
    count: usize,
}

#[derive(Component)]
pub struct Poll {}

#[derive(Component)]
pub struct Bar {}

pub fn spawn_gate_system(
    mut commands: Commands,
    centipede_container: Res<CentipedeContainer>,
    time: Res<Time>,
    mut gates_info: ResMut<GatesInfo>,
    head_query: Query<&Position, With<head::Head>>,
    handles: Res<asset::Handles>,
    bar_query: Query<(&Position, &Sprite), With<Bar>>,
) -> Option<()> {
    if time.elapsed_seconds_f64() / GATE_SPAWN_PER_SECONDS > gates_info.count as f64 {
        let head_position = head_query.get(centipede_container.head_entity()?).ok()?;

        let bars: Vec<(&Position, f32)> = bar_query
            .iter()
            .map(|(position, sprite)| (position, sprite.custom_size.unwrap().x))
            .collect();

        let length = GATE_MIN_WIDTH + random::<f32>() * (GATE_MAX_WIDTH - GATE_MIN_WIDTH);
        if let Some(position) = gate_position(length, &bars, head_position) {
            gates_info.count += 1;

            commands
                .spawn(handles.gate_bundle(length))
                .insert(Bar {})
                .insert(position)
                .with_children(|mut bar| {
                    spawn_poll(&mut bar, length, &handles);
                    spawn_poll(&mut bar, -length, &handles);
                });
        }
    }
    None
}

fn spawn_poll(commands: &mut ChildBuilder, length: f32, handles: &Res<asset::Handles>) {
    commands
        .spawn(handles.gate_poll_mesh_bundle(length))
        .insert(Poll {});
}

fn gate_position(
    length: f32,
    gates: &Vec<(&Position, f32)>,
    head_position: &Position,
) -> Option<Position> {
    for _ in 0..10 {
        let position = Position {
            x: random::<f32>() * (BOARD_X_SIZE - length) - (BOARD_X_BORDER - length / 2.0),
            y: random::<f32>() * (BOARD_Y_SIZE - length) - (BOARD_Y_BORDER - length / 2.0),
            z: GATE_Z,
            visible: true,
        };

        if head_position.distance(&position) > GATE_NOT_SPAWN_DISTANCE_TO_HEAD
            && gates.iter().all(|(other_position, other_length)| {
                position.distance(other_position) > (length + other_length) / 2.0
            })
        {
            return Some(position);
        }
    }
    None
}

pub fn on_game_start(
    mut commands: Commands,
    mut start_reader: EventReader<event::GameStart>,
    bar_query: Query<Entity, With<Bar>>,
) {
    for _ in start_reader.read() {
        for entity in bar_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
