use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct LivingTail {
    pub direction: Vec2,
    pub index: usize,
}

#[derive(Component)]
pub struct PurgedTail {
    direction: Vec2,
    removed_at: f64,
    speed: f32,
}

pub fn on_game_start(mut commands: Commands, mut start_reader: EventReader<event::GameStart>) {
    for _ in start_reader.read() {
        for index in 0..INITIAL_CENTIPEDE_LENGTH {
            tail_bundle(&mut commands, index);
        }
    }
}

pub fn on_through_gate(
    mut commands: Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    mut through_gate_reader: EventReader<event::ThroughGate>,
) -> Option<()> {
    let centipede = centipede_container.alive_mut()?;

    for _ in through_gate_reader.read() {
        tail_bundle(&mut commands, centipede.tail_count);
        centipede.tail_count += 1;
    }
    None
}

pub fn tail_bundle(commands: &mut Commands, index: usize) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: TAIL_COLOR,
                custom_size: Some(TAIL_SIZE),
                ..default()
            },
            ..Default::default()
        })
        .insert(Position::default(false, CENTIPEDE_Z))
        .insert(LivingTail {
            index,
            direction: Vec2::default(),
        });
}

pub fn on_miss(
    mut commands: Commands,
    mut centipede_container: ResMut<CentipedeContainer>,
    time: Res<Time>,
    mut eat_tail_reader: EventReader<event::EatTail>,
    mut crush_poll_reader: EventReader<event::CrushPoll>,
    mut living_tail_query: Query<(Entity, &LivingTail, &mut Sprite)>,
) -> Option<()> {
    let centipede = centipede_container.alive_mut()?;

    for _ in crush_poll_reader.read() {
        let original_count = centipede.tail_count;
        centipede.tail_count = (centipede.tail_count as f32 / 2.0).floor() as usize;
        purge_tail(
            &mut commands,
            &time,
            &centipede,
            original_count,
            &mut living_tail_query,
        )
    }

    for event in eat_tail_reader.read() {
        if event.tail_index >= centipede.tail_count {
            continue;
        }
        let original_count = centipede.tail_count;
        centipede.tail_count = event.tail_index;

        purge_tail(
            &mut commands,
            &time,
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
    centipede: &Alive,
    original_count: usize,
    tail_query: &mut Query<(Entity, &LivingTail, &mut Sprite)>,
) {
    if original_count <= centipede.tail_count {
        return;
    }

    for (entity, tail, mut sprite) in tail_query.iter_mut() {
        if tail.index < centipede.tail_count {
            continue;
        }
        let purged_index_ratio = if original_count > tail.index {
            (original_count - tail.index) as f32
                / (original_count - centipede.tail_count + 1) as f32
        } else {
            0.0
        };

        sprite.color = PURGED_COLOR;
        commands
            .entity(entity)
            .remove::<LivingTail>()
            .insert(PurgedTail {
                direction: tail.direction,
                removed_at: time.elapsed_seconds_f64() + 2.5,
                speed: centipede.speed * purged_index_ratio,
            });
    }
}

pub fn move_tail_system(
    centipede_container: Res<CentipedeContainer>,
    mut tail_query: Query<(&mut Position, &mut Transform, &mut LivingTail)>,
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

    for (mut position, mut transform, mut tail) in tail_query.iter_mut() {
        match tail_positions.get(tail.index) {
            Some(tail_position) => {
                tail.direction = Vec2 {
                    x: tail_position.x - position.x,
                    y: tail_position.y - position.y,
                };
                position.x = tail_position.x;
                position.y = tail_position.y;
                position.visible = true;
                if tail.index > 0 {
                    match tail_positions.get(tail.index - 1) {
                        Some(prev_tail_position) => {
                            transform.rotation = tail_position.head_to(*prev_tail_position);
                        }
                        None => {}
                    }
                } else {
                    if let Some(head_position) = centipede.position_history.last() {
                        transform.rotation = tail_position.head_to(*head_position);
                    }
                }
            }
            None => {}
        }
    }
    None
}
pub fn purged_tail_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &PurgedTail)>,
) {
    for (entity, mut position, purged_tail) in query.iter_mut() {
        if time.elapsed_seconds_f64() > purged_tail.removed_at {
            commands.entity(entity).despawn_recursive();
        } else {
            position.move_to_with_sec(
                purged_tail.direction,
                purged_tail.speed,
                time.delta_seconds(),
            );
        }
    }
}
