use crate::*;
use rand::prelude::random;
use std::f32::consts::PI;

pub struct ModPlugin {}

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ModResources>()
            .init_resource::<GatesInfo>()
            .add_system_to_stage(
                CoreStage::Update,
                spawn_gate_system.system().chain(void.system()),
            )
            .add_system_to_stage(MyStage::ReceiveEvent, on_game_start.system());
    }
}

struct ModResources {
    poll_mesh: Handle<Mesh>,
    poll_material: Handle<StandardMaterial>,
    bar_mesh: Handle<Mesh>,
    bar_material: Handle<StandardMaterial>,
}

impl FromWorld for ModResources {
    fn from_world(world: &mut World) -> Self {
        let mut mesh = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let poll_mesh = mesh.add(Mesh::from(shape::Icosphere {
            radius: POLL_SIZE,
            subdivisions: 5,
        }));
        let bar_mesh = mesh.add(Mesh::from(shape::Cube { size: 1.0 }));

        let mut material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let poll_material = material.add(POLL_COLOR.into());
        let bar_material = material.add(BAR_COLOR.into());

        Self {
            poll_mesh,
            poll_material,
            bar_mesh,
            bar_material,
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
    mut commands: Commands,
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
            .spawn_bundle(ContainerBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., INVISIBLE_OBJECT_Z),
                    rotation: Quat::from_rotation_z(random::<f32>() * PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Gate {})
            .insert(position)
            .id();

        spawn_poll(&mut commands, &resources, gate, length);
        spawn_poll(&mut commands, &resources, gate, -length);

        commands
            .spawn_bundle(PbrBundle {
                mesh: resources.bar_mesh.clone(),
                material: resources.bar_material.clone(),
                transform: Transform {
                    scale: Vec3::new(length, BAR_DIAMETER, BAR_DIAMETER),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bar {})
            .insert(Parent(gate));
    }
    None
}

fn spawn_poll(commands: &mut Commands, resources: &Res<ModResources>, gate: Entity, length: f32) {
    // https://github.com/bevyengine/bevy/blob/master/examples/ecs/hierarchy.rs
    commands
        .spawn_bundle(PbrBundle {
            mesh: resources.poll_mesh.clone(),
            material: resources.poll_material.clone(),
            transform: Transform::from_translation(Vec3::new(length / 2.0, 0.0, 0.0)),
            global_transform: GlobalTransform::from_translation(Vec3::new(
                0.,
                0.,
                constants::INVISIBLE_OBJECT_Z,
            )),
            ..Default::default()
        })
        .insert(Poll {})
        .insert(Parent(gate));
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
    mut commands: Commands,
    mut reader: EventReader<GameStart>,
    query: Query<Entity, With<Gate>>,
) {
    for _ in reader.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
