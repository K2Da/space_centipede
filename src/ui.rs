use crate::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct ModPlugin;

// スコア等の文字表示
impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<Status>()
            .add_startup_system(setup.system())
            .add_system(on_game_start.system())
            .add_system(on_through_gate.system())
            .add_system(score_update_system.system())
            .add_system(fps_update_system.system());
    }
}

struct FpsText;

struct ScoreText;

#[derive(Default)]
struct Status {
    score: usize,
    high_score: usize,
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT);
    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(100.0),
                    top: Val::Percent(4.0),
                    bottom: Val::Percent(96.0),
                },
                align_self: AlignSelf::Baseline,
                ..Default::default()
            },
            text: Text {
                value: FPS_PREFIX.to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: FPS_SIZE,
                    color: FPS_COLOR,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText)
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(30.0),
                    right: Val::Percent(70.0),
                    top: Val::Percent(4.0),
                    bottom: Val::Percent(96.0),
                },
                ..Default::default()
            },
            text: Text {
                value: SCORE_PREFIX.to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: SCORE_SIZE,
                    color: SCORE_COLOR,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(ScoreText);
}

fn on_through_gate(
    centipede_container: Res<CentipedeContainer>,
    mut status: ResMut<Status>,
    events: Res<Events<event::ThroughGate>>,
    mut reader: Local<EventReader<event::ThroughGate>>,
) {
    if let Centipede::Alive(centipede) = &centipede_container.centipede {
        for _ in reader.iter(&events) {
            status.score +=
                (centipede.tail_count as f32 * centipede.speed / 100.0).floor() as usize;
            if status.score >= status.high_score {
                status.high_score = status.score;
            }
        }
    }
}

fn on_game_start(
    mut status: ResMut<Status>,
    events: Res<Events<event::GameStart>>,
    mut reader: Local<EventReader<event::GameStart>>,
) {
    for _ in reader.iter(&events) {
        status.score = 0;
    }
}

fn fps_update_system(
    diagnostics: Res<Diagnostics>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in fps_query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("{:} {:.2}", FPS_PREFIX, average).into();
            }
        }
    }
}

fn score_update_system(
    centipede_container: Res<CentipedeContainer>,
    status: Res<Status>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
) {
    if let Centipede::Alive(centipede) = &centipede_container.centipede {
        for mut text in score_query.iter_mut() {
            text.value = format!(
                "{:} {:.0}              {:} {:.0}              {:} {:.0}              {:} {:.0}",
                SPEED_PREFIX,
                centipede.speed,
                TAIL_PREFIX,
                centipede.tail_count,
                SCORE_PREFIX,
                status.score,
                HIGH_SCORE_PREFIX,
                status.high_score,
            )
            .into();
        }
    }
}
