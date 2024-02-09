use crate::*;

pub struct ModPlugin;

// イベントの初期化と、ゲーム全体のサイクルに関わるイベントの送信
impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CentipedeContainer>()
            .add_event::<GameStart>()
            .add_event::<GameOver>()
            .add_event::<Tap>()
            .add_event::<CrushPoll>()
            .add_event::<ThroughGate>()
            .add_event::<EatTail>();
    }
}

// ゲーム開始
#[derive(Event)]
pub struct GameStart {}

// 死亡
#[derive(Event)]
pub struct GameOver {
    pub head_entity: Entity,
}

// 門の脇の柱に激突
#[derive(Event)]
pub struct CrushPoll {}

// 門を通過
#[derive(Event)]
pub struct ThroughGate {}

// 尾にぶつかる
#[derive(Event)]
pub struct EatTail {
    pub tail_index: usize,
}

#[derive(Event)]
pub struct Tap {}

// 尾が無くなったら終わり
pub fn game_over_system(
    time: Res<Time>,
    mut centipede_container: ResMut<CentipedeContainer>,
    mut game_over_events: ResMut<Events<GameOver>>,
) -> Option<()> {
    let centipede = centipede_container.alive()?;
    if centipede.tail_count <= 0 {
        game_over_events.send(GameOver {
            head_entity: centipede.head_entity,
        });
        centipede_container.centipede = Centipede::Dead(time.elapsed_seconds_f64());
    }
    None
}

// ゲームが終わって規定の時間が経ったら、再開。起動時の処理も同じ
pub fn game_start_system(
    time: Res<Time>,
    centipede_container: Res<CentipedeContainer>,
    mut game_start_events: bevy::prelude::ResMut<Events<event::GameStart>>,
) {
    if let Centipede::Dead(dead_at) = centipede_container.centipede {
        if dead_at == 0.0 || dead_at < time.elapsed_seconds_f64() - 2.0 {
            game_start_events.send(event::GameStart {});
        }
    }
}
