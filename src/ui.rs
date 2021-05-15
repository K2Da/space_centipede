use crate::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct ModPlugin;

// スコア等の文字表示
impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<Status>()
            .add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::Last, on_game_start.system())
            .add_system_to_stage(CoreStage::Last, on_through_gate.system())
            .add_system_to_stage(MyStage::PreRender, score_update_system.system())
            .add_system_to_stage(MyStage::PreRender, fps_update_system.system());
    }
}

struct FpsText;

struct ScoreText;

#[derive(Default)]
struct Status {
    score: usize,
    high_score: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT);
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
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
            text: Text::with_section(
                FPS_PREFIX.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: FPS_SIZE,
                    color: FPS_COLOR,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(FpsText);

    commands
        .spawn_bundle(TextBundle {
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
            text: Text::with_section(
                SCORE_PREFIX.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: SCORE_SIZE,
                    color: SCORE_COLOR,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(ScoreText);
}

fn on_through_gate(
    centipede_container: Res<CentipedeContainer>,
    mut status: ResMut<Status>,
    mut reader: EventReader<ThroughGate>,
) {
    if let Centipede::Alive(centipede) = &centipede_container.centipede {
        for _ in reader.iter() {
            status.score +=
                (centipede.tail_count as f32 * centipede.speed / 100.0).floor() as usize;
            if status.score >= status.high_score {
                status.high_score = status.score;
            }
        }
    }
}

fn on_game_start(mut status: ResMut<Status>, mut reader: EventReader<GameStart>) {
    for _ in reader.iter() {
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
                text.sections.first_mut().unwrap().value =
                    format!("{:} {:.2}", FPS_PREFIX, average).into();
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
            text.sections.first_mut().unwrap().value = format!(
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
