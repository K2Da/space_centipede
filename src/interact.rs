use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn head_and_gate_system(
    mut commands: Commands,
    mut through_gate_events: ResMut<Events<event::ThroughGate>>,
    mut crush_gate_events: ResMut<Events<event::CrushPoll>>,
    centipede_container: Res<CentipedeContainer>,
    head_query: Query<&GlobalTransform, With<head::Head>>,
    bar_query: Query<(Entity, &Children), With<gate::Bar>>,
    poll_query: Query<&GlobalTransform, With<gate::Poll>>,
) -> Option<()> {
    let centipede = centipede_container.alive()?;
    let head_translation = head_query.get(centipede.head_entity).ok()?.translation();

    for (bar, children) in bar_query.iter() {
        let poll_translations: Vec<Vec3> = children
            .iter()
            .flat_map(|poll_entity| poll_query.get(*poll_entity))
            .map(|poll_transform| poll_transform.translation())
            .collect();

        // Gateの両脇にあたったらミス
        for poll_translation in &poll_translations {
            if distance_2d(head_translation, *poll_translation) <= POLL_SIZE + HEAD_SIZE {
                commands.entity(bar).despawn_recursive();
                crush_gate_events.send(event::CrushPoll {});
            }
        }

        // 門の中をくぐったら、OK
        if let (Some(head1), Some(head2), Some(poll1), Some(poll2)) = (
            centipede.position_history.last(),
            centipede
                .position_history
                .get(centipede.position_history.len() - 2),
            poll_translations.get(0),
            poll_translations.get(1),
        ) {
            if intersection(head1, head2, &(*poll1).into(), &(*poll2).into()) {
                commands.entity(bar).despawn_recursive();
                through_gate_events.send(event::ThroughGate {});
            }
        }
    }
    None
}

pub fn head_and_tail_system(
    mut eat_tail_events: ResMut<Events<event::EatTail>>,
    centipede_container: Res<CentipedeContainer>,
    head_query: Query<&GlobalTransform, With<head::Head>>,
    tail_query: Query<(&tail::LivingTail, &GlobalTransform, &Position)>,
) -> Option<()> {
    // 生きてるときだけチェックするので、head_entity()を使っている
    let head_translation = head_query
        .get(centipede_container.head_entity()?)
        .ok()?
        .translation();

    for (tail, tail_global_transform, tail_position) in tail_query.iter() {
        if tail_position.visible
            && distance_2d(head_translation, tail_global_transform.translation()) <= HEAD_SIZE
        {
            eat_tail_events.send(event::EatTail {
                tail_index: tail.index,
            });
        }
    }
    None
}
