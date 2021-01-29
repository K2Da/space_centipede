use crate::*;
use rand::prelude::random;
use std::f32::consts::PI;

pub struct ModPlugin {}

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ModResources>()
            .init_resource::<GatesInfo>()
            .add_system_to_stage(
                stage::UPDATE,
                spawn_gate_system.system().chain(void.system()),
            )
            .add_system_to_stage(stage::RECEIVE_EVENT, on_game_start.system());
    }
}

struct ModResources {
    poll_mesh: Handle<Mesh>,
    poll_material: Handle<StandardMaterial>,
    bar_mesh: Handle<Mesh>,
    bar_material: Handle<StandardMaterial>,
}

impl FromResources for ModResources {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();

        Self {
            poll_mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: POLL_SIZE,
                subdivisions: 5,
            })),
            poll_material: materials.add(POLL_COLOR.into()),
            bar_mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            bar_material: materials.add(BAR_COLOR.into()),
        }
    }
}

#[derive(Default)]
pub struct GatesInfo {
    count: usize,
}

pub struct Gate {}

pub struct Poll {}

pub struct Bar {}

fn spawn_gate_system(
    commands: &mut Commands,
    centipede_container: Res<CentipedeContainer>,
    time: Res<Time>,
    resources: Res<ModResources>,
    mut gates_info: ResMut<GatesInfo>,
    head_query: Query<&Position, With<head::Head>>,
) -> Option<()> {
    let head_position = head_query.get(centipede_container.head_entity()?).ok()?;

    if time.seconds_since_startup() / GATE_SPAWN_PER_SECONDS > gates_info.count as f64 {
        gates_info.count += 1;
        let length = GATE_MIN_WIDTH + random::<f32>() * (GATE_MAX_WIDTH - GATE_MIN_WIDTH);
        let position = gate_position(length, head_position);

        let gate = commands
            .spawn(ContainerBundle {
                transform: Transform {
                    translation: constants::INVISIBLE_POSITION,
                    rotation: Quat::from_rotation_z(random::<f32>() * PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Gate {})
            .with(position)
            .current_entity()
            .unwrap();

        spawn_poll(commands, &resources, gate, length);
        spawn_poll(commands, &resources, gate, -length);
        // childrenはpositionもってないので、transformationから算出する

        commands
            .spawn(PbrBundle {
                mesh: resources.bar_mesh.clone(),
                material: resources.bar_material.clone(),
                transform: Transform {
                    scale: Vec3 {
                        x: length,
                        y: BAR_DIAMETER,
                        z: BAR_DIAMETER,
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Bar {})
            .with(Parent(gate));
    }
    None
}

fn spawn_poll(commands: &mut Commands, resources: &Res<ModResources>, gate: Entity, length: f32) {
    // https://github.com/bevyengine/bevy/blob/master/examples/ecs/hierarchy.rs
    commands
        .spawn(PbrBundle {
            mesh: resources.poll_mesh.clone(),
            material: resources.poll_material.clone(),
            transform: Transform::from_translation(Vec3 {
                x: length / 2.0,
                y: 0.0,
                z: 0.0,
            }),
            global_transform: GlobalTransform::from_translation(constants::INVISIBLE_POSITION),
            ..Default::default()
        })
        .with(Poll {})
        .with(Parent(gate));
}

fn gate_position(length: f32, head_position: &Position) -> Position {
    loop {
        let position = Position {
            x: random::<f32>() * (constants::BOARD_X_SIZE - length)
                - (constants::BOARD_X_BORDER - length / 2.0),
            y: random::<f32>() * (constants::BOARD_Y_SIZE - length)
                - (constants::BOARD_Y_BORDER - length / 2.0),
            visible: true,
        };

        if head_position.distance(&position) > GATE_NOT_SPAWN_DISTANCE_TO_HEAD {
            return position;
        }
    }
}

fn on_game_start(
    commands: &mut Commands,
    (events, mut reader): (Res<Events<GameStart>>, Local<EventReader<GameStart>>),
    query: Query<Entity, With<Gate>>,
) {
    for _ in reader.iter(&events) {
        for entity in query.iter() {
            commands.despawn_recursive(entity);
        }
    }
}
