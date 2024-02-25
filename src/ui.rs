use crate::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub struct ModPlugin;

// スコア等の文字表示
impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Status>()
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .insert_resource(Time::<Fixed>::from_seconds(1.0))
            .add_systems(Update, button_system);
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
fn text_style() -> TextStyle {
    TextStyle {
        font_size: UI_FONT_SIZE,
        color: UI_TEXT_COLOR,
        ..default()
    }
}
pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = Style {
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };
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
                    TextBundle::from_section(FPS_PREFIX.to_string(), text_style())
                        .with_style(style.clone()),
                    Label,
                ))
                .insert(FpsText);
            parent
                .spawn((
                    TextBundle::from_section(SCORE_PREFIX.to_string(), text_style())
                        .with_style(style.clone()),
                    Label,
                ))
                .insert(ScoreText);
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(UI_FONT_SIZE + 5.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Pause", text_style()));
                });
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
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections = vec![TextSection {
                value: format!("{:} {:.2}", FPS_PREFIX, fps).into(),
                style: text_style(),
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
            text.sections = vec![TextSection {
                value: format!(
                    "{:} {:.0}  {:} {:.0}  {:} {:.0}  {:} {:.0}",
                    SPEED_PREFIX,
                    centipede.speed,
                    TAIL_PREFIX,
                    centipede.tail_count,
                    SCORE_PREFIX,
                    status.score,
                    HIGH_SCORE_PREFIX,
                    status.high_score
                )
                .into(),
                style: text_style(),
            }];
        }
    }
}

fn button_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::Menu);
            }
            _ => (),
        }
    }
}
