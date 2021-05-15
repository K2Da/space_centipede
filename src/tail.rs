use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ModResources>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                move_tail_system.system().chain(void.system()),
            )
            .add_system_to_stage(CoreStage::PostUpdate, purged_tail_system.system())
            .add_system_to_stage(CoreStage::PostUpdate, rotate_tail_system.system())
            .add_system_to_stage(MyStage::ReceiveEvent, on_game_start.system())
            .add_system_to_stage(
                MyStage::ReceiveEvent,
                on_through_gate.system().chain(void.system()),
            )
            .add_system_to_stage(MyStage::ReceiveEvent, on_miss.system().chain(void.system()));
    }
}

struct ModResources {
    forward_axis: Vec3,
    base_quaternion: Quat,
    spin_axis: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    purged_material: Handle<StandardMaterial>,
}

impl FromWorld for ModResources {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .unwrap()
            .add(Mesh::from(shape::Cube { size: TAIL_SIZE }));
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material = materials.add(TAIL_COLOR.into());
        let purged_material = materials.add(PURGED_COLOR.into());

        Self {
            forward_axis: Vec3::new(0.0, 0.0, -1.0),
            base_quaternion: Quat::from_axis_angle(
                Vec3::new(1.0, -1.0, 0.0).normalize(),
                -Vec2::new(1.0, 0.0).angle_between(Vec2::new(1.0, 1.0)),
            ),
            spin_axis: Vec3::new(1.0, 1.0, 1.0).normalize(),
            mesh,
            material,
            purged_material,
        }
    }
}

struct Spinner {
    direction: Vec2,
    margin: f64,
}

pub struct LivingTail {
    pub index: usize,
}

struct PurgedTail {
    remove_at: f64,
    speed: f32,
}

fn on_game_start(
    mut commands: Commands,
    resources: Res<ModResources>,
    mut reader: EventReader<GameStart>,
) {
    for _ in reader.iter() {
        for i in 0..INITIAL_CENTIPEDE_LENGTH {
            let index = i;
            spawn_tail(&mut commands, &resources, index);
        }
    }
}

fn on_through_gate(
    mut commands: Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    resources: Res<ModResources>,
    mut reader: EventReader<ThroughGate>,
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;

    for _ in reader.iter() {
        let index = centipede.tail_count;
        spawn_tail(&mut commands, &resources, index);
        centipede.tail_count += 1;
    }
    None
}

fn spawn_tail(commands: &mut Commands, resources: &Res<ModResources>, index: usize) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: resources.mesh.clone(),
            material: resources.material.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., INVISIBLE_OBJECT_Z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position::default(false))
        .insert(LivingTail { index })
        .insert(Spinner {
            direction: Vec2::new(0.0, 0.0),
            margin: index as f64 * 0.2,
        });
}

fn on_miss(
    mut commands: Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    time: Res<Time>,
    resources: Res<ModResources>,
    mut eat_tail_reader: EventReader<EatTail>,
    mut crush_poll_reader: EventReader<CrushPoll>,
    mut living_tail_query: Query<(Entity, &LivingTail)>,
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;

    for _ in crush_poll_reader.iter() {
        let original_count = centipede.tail_count;
        centipede.tail_count = (centipede.tail_count as f32 / 2.0).floor() as usize;
        purge_tail(
            &mut commands,
            &time,
            &resources,
            &centipede,
            original_count,
            &mut living_tail_query,
        )
    }

    for event in eat_tail_reader.iter() {
        if event.tail_index >= centipede.tail_count {
            continue;
        }
        let original_count = centipede.tail_count;
        centipede.tail_count = event.tail_index;

        purge_tail(
            &mut commands,
            &time,
            &resources,
            &centipede,
            original_count,
            &mut living_tail_query,
        )
    }
    None
}

fn purge_tail(
    commands: &mut Commands,
    time: &Time,
    resources: &ModResources,
    centipede: &Alive,
    original_count: usize,
    tail_query: &mut Query<(Entity, &LivingTail)>,
) {
    if original_count <= centipede.tail_count {
        return;
    }

    for (entity, tail) in tail_query.iter_mut() {
        if tail.index < centipede.tail_count {
            continue;
        }
        let purged_index_ratio = if original_count > tail.index {
            (original_count - tail.index) as f32
                / (original_count - centipede.tail_count + 1) as f32
        } else {
            0.0
        };

        commands
            .entity(entity)
            .remove::<(LivingTail, Handle<StandardMaterial>)>()
            .insert(PurgedTail {
                remove_at: time.seconds_since_startup() + 2.5,
                speed: centipede.speed * purged_index_ratio,
            })
            .insert(resources.purged_material.clone());
    }
}

fn move_tail_system(
    centipede_container: Res<CentipedeContainer>,
    mut tail_query: Query<(&mut Position, &LivingTail, &mut Spinner)>,
) -> Option<()> {
    let centipede = centipede_container.alive()?;

    let mut tail_positions = vec![];
    let mut prev_position = None;
    let mut distance = 0.0;

    'outer: for position in centipede.position_history.iter().rev() {
        match prev_position {
            Some(prev) => {
                let current_distance = position.distance(&prev);
                distance += current_distance;
                while distance >= TAIL_DISTANCE {
                    distance = distance - TAIL_DISTANCE;
                    tail_positions.push(prev.forward_to(position, current_distance - distance));
                    if tail_positions.len() > centipede.tail_count {
                        break 'outer;
                    }
                }
            }
            None => {}
        }
        prev_position = Some(position.clone());
    }

    for (mut position, tail, mut spinner) in tail_query.iter_mut() {
        match tail_positions.get(tail.index) {
            Some(tail_position) => {
                spinner.direction =
                    Vec2::new(tail_position.x - position.x, tail_position.y - position.y);
                position.x = tail_position.x;
                position.y = tail_position.y;
                position.visible = true;
            }
            None => {}
        }
    }
    None
}

fn rotate_tail_system(
    time: Res<Time>,
    rotations: Res<ModResources>,
    mut query: Query<(&mut Transform, &Spinner)>,
) {
    for (mut transform, spinner) in query.iter_mut() {
        // 進行方向に合わせる
        let to_forward = Quat::from_axis_angle(
            rotations.forward_axis,
            spinner.direction.angle_between(Vec2::new(1.0, 1.0)),
        );

        // 頂点が前方に来るように
        let tilt = rotations.base_quaternion;

        // 時間経過で回転させる
        let spin = Quat::from_axis_angle(
            rotations.spin_axis,
            (time.seconds_since_startup() * 1.0 + spinner.margin) as f32,
        );

        // 合成する
        transform.rotation = to_forward * tilt * spin;
    }
}

fn purged_tail_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &mut Spinner, &PurgedTail)>,
) {
    for (entity, mut position, spinner, purged_tail) in query.iter_mut() {
        if time.seconds_since_startup() > purged_tail.remove_at {
            commands.entity(entity).despawn_recursive();
        } else {
            position.move_to_with_sec(spinner.direction, purged_tail.speed, time.delta_seconds());
        }
    }
}
