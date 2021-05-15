use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            MyStage::SendEvent,
            head_and_gate_system.system().chain(void.system()),
        )
        .add_system_to_stage(
            MyStage::SendEvent,
            head_and_tail_system.system().chain(void.system()),
        );
    }
}

fn head_and_gate_system(
    mut commands: Commands,
    mut through_gate_writer: EventWriter<ThroughGate>,
    mut crush_gate_writer: EventWriter<CrushPoll>,
    centipede_container: Res<CentipedeContainer>,
    head_query: Query<&GlobalTransform, With<head::Head>>,
    gate_query: Query<(Entity, &Children), With<gate::Gate>>,
    poll_query: Query<&GlobalTransform, With<gate::Poll>>,
) -> Option<()> {
    let centipede = centipede_container.alive()?;
    let head_translation = head_query.get(centipede.head_entity).ok()?.translation;

    for (gate, children) in gate_query.iter() {
        let poll_translations: Vec<Vec3> = children
            .iter()
            .flat_map(|poll_entity| poll_query.get(*poll_entity))
            .map(|poll_transform| poll_transform.translation)
            .collect();

        // Gateの両脇にあたったらミス
        for poll_translation in &poll_translations {
            if head_translation.distance(poll_translation.clone())
                <= constants::POLL_SIZE + constants::HEAD_SIZE
            {
                // ここで消さないと次のフレームで再度衝突する
                commands.entity(gate).despawn_recursive();
                // このイベント使ってないが
                crush_gate_writer.send(CrushPoll {});
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
                commands.entity(gate).despawn_recursive();
                through_gate_writer.send(ThroughGate {});
            }
        }
    }
    None
}

fn head_and_tail_system(
    mut eat_tail_writer: EventWriter<EatTail>,
    centipede_container: Res<CentipedeContainer>,
    head_query: Query<&GlobalTransform, With<head::Head>>,
    tail_query: Query<(&tail::LivingTail, &GlobalTransform)>,
) -> Option<()> {
    let head_translation = head_query
        .get(centipede_container.head_entity()?)
        .ok()?
        .translation;

    for (tail, tail_global_transform) in tail_query.iter() {
        let tail_translation = tail_global_transform.translation;

        if head_translation.distance(tail_translation.clone()) <= constants::HEAD_SIZE {
            eat_tail_writer.send(EatTail {
                tail_index: tail.index,
            });
        }
    }
    None
}
