use crate::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub struct ModPlugin;

// スコア等の文字表示
impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Status>()
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource, Default)]
pub struct Status {
    score: usize,
    high_score: usize,
}

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    TextBundle::from_section(
                        FPS_PREFIX.to_string(),
                        TextStyle {
                            font_size: FPS_SIZE,
                            color: FPS_COLOR,
                            font: font.clone(),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    }),
                    Label,
                ))
                .insert(FpsText);
            parent
                .spawn((
                    TextBundle::from_section(
                        SCORE_PREFIX.to_string(),
                        TextStyle {
                            font_size: SCORE_SIZE,
                            color: SCORE_COLOR,
                            font,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    }),
                    Label,
                ))
                .insert(ScoreText);
        });
}

pub fn on_through_gate(
    centipede_container: Res<CentipedeContainer>,
    mut status: ResMut<Status>,
    mut gate_reader: EventReader<event::ThroughGate>,
) {
    if let Centipede::Alive(centipede) = &centipede_container.centipede {
        for _ in gate_reader.read() {
            status.score +=
                (centipede.tail_count as f32 * centipede.speed / 100.0).floor() as usize;
            if status.score >= status.high_score {
                status.high_score = status.score;
            }
        }
    }
}

pub fn on_game_start(mut status: ResMut<Status>, mut start_reader: EventReader<event::GameStart>) {
    for _ in start_reader.read() {
        status.score = 0;
    }
}

pub fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in fps_query.iter_mut() {
        if let Some(fps) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections = vec![TextSection {
                value: format!("{:} {:.2}", FPS_PREFIX, fps).into(),
                style: TextStyle {
                    font_size: SCORE_SIZE,
                    color: SCORE_COLOR,
                    ..Default::default()
                },
            }];
        }
    }
}

pub fn score_update_system(
    centipede_container: Res<CentipedeContainer>,
    status: Res<Status>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
) {
    if let Centipede::Alive(centipede) = &centipede_container.centipede {
        for mut text in score_query.iter_mut() {
            text.sections = vec![
            TextSection {
                value: format!(
                    "{:} {:.0}              {:} {:.0}              {:} {:.0}              {:} {:.0}",
                    SPEED_PREFIX, centipede.speed, TAIL_PREFIX, centipede.tail_count, SCORE_PREFIX, status.score, HIGH_SCORE_PREFIX, status.high_score
                ).into(),
                style: TextStyle {
                    font_size: SCORE_SIZE,
                    color: SCORE_COLOR,
                    ..Default::default()
                },
            }];
        }
    }
}
