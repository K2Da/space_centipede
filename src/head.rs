use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ModResources>()
            .add_startup_system(setup.system())
            .add_system_to_stage(
                stage::PRE_UPDATE,
                select_movement_system.system().chain(void.system()),
            )
            .add_system(move_head_system.system().chain(void.system()))
            .add_system_to_stage(stage::LAST, on_game_start.system())
            .add_system_to_stage(stage::LAST, on_game_over.system());
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Head {}

struct CenterMarker {}

struct ModResources {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromResources for ModResources {
    fn from_resources(resources: &Resources) -> Self {
        Self {
            mesh: resources
                .get_mut::<Assets<Mesh>>()
                .unwrap()
                .add(Mesh::from(shape::Icosphere {
                    radius: HEAD_SIZE,
                    subdivisions: 5,
                })),
            material: resources
                .get_mut::<Assets<StandardMaterial>>()
                .unwrap()
                .add(HEAD_COLOR.into()),
        }
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: MARKER_SIZE,
                subdivisions: 5,
            })),
            material: materials.add(MARKER_COLOR.into()),
            ..Default::default()
        })
        .with(CenterMarker {})
        .with(Position::default(false));
}

fn select_movement_system(
    mut centipede_container: ResMut<CentipedeContainer>,
    cursor_state: Res<input::CursorState>,
    mut marker_query: Query<&mut Position, With<CenterMarker>>,
    head_query: Query<&Position, With<Head>>,
) -> Option<()> {
    let mut centipede = centipede_container.alive_mut()?;
    let position = head_query.get(centipede.head_entity).ok()?;

    let mut circular = false;
    match centipede.movement {
        Movement::Circular(_) => {
            if !cursor_state.left_pressed {
                centipede.movement = Movement::Linear(centipede.last_move);
            }
            circular = true;
        }
        Movement::Linear(_) => {
            let vec = Vec2 {
                y: -cursor_state.position.x + position.x,
                x: cursor_state.position.y - position.y,
            };
            let inner_product = vec.x * centipede.last_move.x + vec.y * centipede.last_move.y;
            if cursor_state.left_pressed {
                centipede.movement = Movement::Circular(CircularMove {
                    center: cursor_state.position.clone(),
                    clockwise: inner_product < 0.0,
                });
            }
        }
    }

    for mut marker in marker_query.iter_mut() {
        // 回転してるときだけ表示
        marker.visible = circular;

        if !circular {
            // 回転してない間は場所だけカーソルに追従する
            marker.x = cursor_state.position.x;
            marker.y = cursor_state.position.y;
        }
    }
    None
}

fn move_head_system(
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
        position.x > constants::BOARD_X_BORDER && centipede.last_move.x > 0.0
            || position.x < -constants::BOARD_X_BORDER && centipede.last_move.x < 0.0,
        position.y > constants::BOARD_Y_BORDER && centipede.last_move.y > 0.0
            || position.y < -constants::BOARD_Y_BORDER && centipede.last_move.y < 0.0,
    );

    if out_x || out_y {
        let Vec2 { x, y } = centipede.last_move;
        centipede.movement = Movement::Linear(Vec2 {
            x: if out_x { -x } else { x },
            y: if out_y { -y } else { y },
        })
    }
}

fn on_game_start(
    commands: &mut Commands,
    resources: Res<ModResources>,
    mut centipede_container: ResMut<CentipedeContainer>,
    (events, mut reader): (Res<Events<GameStart>>, Local<EventReader<GameStart>>),
) {
    for _ in reader.iter(&events) {
        centipede_container.centipede = Centipede::Alive(Alive::default(
            commands
                .spawn(PbrBundle {
                    mesh: resources.mesh.clone(),
                    material: resources.material.clone(),
                    ..Default::default()
                })
                .with(Head {})
                .with(Position::default(true))
                .current_entity()
                .unwrap(),
        ));
    }
}

fn on_game_over(
    commands: &mut Commands,
    events: Res<Events<GameOver>>,
    mut reader: Local<EventReader<GameOver>>,
) {
    for event in reader.iter(&events) {
        commands.despawn(event.head_entity);
    }
}
