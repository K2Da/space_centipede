use crate::*;

pub struct ModPlugin;

// イベントの初期化と、ゲーム全体のサイクルに関わるイベントの送信
impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CentipedeContainer>()
            .add_event::<GameStart>()
            .add_event::<GameOver>()
            .add_event::<CrushPoll>()
            .add_event::<ThroughGate>()
            .add_event::<EatTail>()
            .add_system_to_stage(stage::POST_UPDATE, game_start_system.system())
            .add_system_to_stage(
                stage::POST_UPDATE,
                game_over_system.system().chain(void.system()),
            );
    }
}

// ゲーム開始
pub struct GameStart {}

// 死亡
pub struct GameOver {
    pub head_entity: Entity,
}

// 門の脇の柱に激突
pub struct CrushPoll {}

// 門を通過
pub struct ThroughGate {}

// 尾にぶつかる
pub struct EatTail {
    pub tail_index: usize,
}

// 尾が無くなったら終わり
fn game_over_system(
    time: Res<Time>,
    mut centipede_container: ResMut<CentipedeContainer>,
    mut game_over_events: ResMut<Events<GameOver>>,
) -> Option<()> {
    let centipede = centipede_container.alive()?;
    if centipede.tail_count <= 0 {
        game_over_events.send(GameOver {
            head_entity: centipede.head_entity,
        });
        centipede_container.centipede = Centipede::Dead(time.seconds_since_startup());
    }
    None
}

// ゲームが終わって規定の時間が経ったら、再開。起動時の処理も同じ
fn game_start_system(
    time: Res<Time>,
    centipede_container: Res<CentipedeContainer>,
    mut game_start_events: ResMut<Events<event::GameStart>>,
) {
    if let Centipede::Dead(dead_at) = centipede_container.centipede {
        if dead_at == 0.0 || dead_at < time.seconds_since_startup() - 2.0 {
            game_start_events.send(event::GameStart {});
        }
    }
}
