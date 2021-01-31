use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ModResources>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                move_tail_system.system().chain(void.system()),
            )
            .add_system_to_stage(stage::POST_UPDATE, purged_tail_system.system())
            .add_system_to_stage(stage::POST_UPDATE, rotate_tail_system.system())
            .add_system_to_stage(stage::RECEIVE_EVENT, on_game_start.system())
            .add_system_to_stage(
                stage::RECEIVE_EVENT,
                on_through_gate.system().chain(void.system()),
            )
            .add_system_to_stage(stage::RECEIVE_EVENT, on_miss.system().chain(void.system()));
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

impl FromResources for ModResources {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        Self {
            forward_axis: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            base_quaternion: Quat::from_axis_angle(
                Vec3 {
                    x: 1.0,
                    y: -1.0,
                    z: 0.0,
                }
                .normalize(),
                -Vec2 { x: 1.0, y: 0.0 }.angle_between(Vec2 { x: 1.0, y: 1.0 }),
            ),
            spin_axis: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            .normalize(),
            mesh: meshes.add(Mesh::from(shape::Cube { size: TAIL_SIZE })),
            material: materials.add(TAIL_COLOR.into()),
            purged_material: materials.add(PURGED_COLOR.into()),
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
    commands: &mut Commands,
    resources: Res<ModResources>,
    (events, mut reader): (Res<Events<GameStart>>, Local<EventReader<GameStart>>),
) {
    for _ in reader.iter(&events) {
        for i in 0..INITIAL_CENTIPEDE_LENGTH {
            spawn_tail(commands, &resources, i);
        }
    }
}

fn on_through_gate(
    commands: &mut Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    resources: Res<ModResources>,
    (events, mut reader): (Res<Events<ThroughGate>>, Local<EventReader<ThroughGate>>),
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;

    for _ in reader.iter(&events) {
        spawn_tail(commands, &resources, centipede.tail_count);
        centipede.tail_count += 1;
    }
    None
}

fn spawn_tail(commands: &mut Commands, resources: &Res<ModResources>, index: usize) {
    commands
        .spawn(PbrBundle {
            mesh: resources.mesh.clone(),
            material: resources.material.clone(),
            transform: Transform {
                translation: constants::INVISIBLE_POSITION,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Position::default(false))
        .with(LivingTail { index })
        .with(Spinner {
            direction: Vec2 { x: 0.0, y: 0.0 },
            margin: index as f64 * 0.2,
        });
}

fn on_miss(
    commands: &mut Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    time: Res<Time>,
    resources: Res<ModResources>,
    (eat_tail_events, mut eat_tail_reader): (Res<Events<EatTail>>, Local<EventReader<EatTail>>),
    (crush_poll_events, mut crush_poll_reader): (
        Res<Events<CrushPoll>>,
        Local<EventReader<CrushPoll>>,
    ),
    mut living_tail_query: Query<(Entity, &LivingTail)>,
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;

    for _ in crush_poll_reader.iter(&crush_poll_events) {
        let original_count = centipede.tail_count;
        centipede.tail_count = (centipede.tail_count as f32 / 2.0).floor() as usize;
        purge_tail(
            commands,
            &time,
            &resources,
            &centipede,
            original_count,
            &mut living_tail_query,
        )
    }

    for event in eat_tail_reader.iter(&eat_tail_events) {
        if event.tail_index >= centipede.tail_count {
            continue;
        }
        let original_count = centipede.tail_count;
        centipede.tail_count = event.tail_index;

        purge_tail(
            commands,
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

        commands.remove::<(LivingTail, Handle<StandardMaterial>)>(entity);
        commands.insert(
            entity,
            (
                PurgedTail {
                    remove_at: time.seconds_since_startup() + 2.5,
                    speed: centipede.speed * purged_index_ratio,
                },
                resources.purged_material.clone(),
            ),
        );
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
                spinner.direction = Vec2 {
                    x: tail_position.x - position.x,
                    y: tail_position.y - position.y,
                };
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
            spinner.direction.angle_between(Vec2 { x: 1.0, y: 1.0 }),
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
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &mut Spinner, &PurgedTail)>,
) {
    for (entity, mut position, spinner, purged_tail) in query.iter_mut() {
        if time.seconds_since_startup() > purged_tail.remove_at {
            commands.despawn_recursive(entity);
        } else {
            position.move_to_with_sec(spinner.direction, purged_tail.speed, time.delta_seconds());
        }
    }
}
