use crate::*;

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_system.system())
            .add_system_to_stage(stage::PRE_RENDER, position_to_translation_system.system());
    }
}

// 背景の碁盤目状のパネルと、ライト、カメラ等を生成
fn setup_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(LIGHT_COORDINATE),
            light: Light {
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 100.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });

    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: PANEL_SIZE - 2.0,
    }));

    let material = materials.add(PANEL_COLOR.into());

    for x in 0..PANEL_X_COUNT {
        for y in 0..PANEL_Y_COUNT {
            block(
                commands,
                mesh.clone(),
                material.clone(),
                x - (PANEL_X_COUNT - 1) / 2,
                y - (PANEL_Y_COUNT - 1) / 2,
            );
        }
    }
}

fn block(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    x: isize,
    y: isize,
) {
    // mesh以外はコピー
    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform {
            translation: Vec3::new(x as f32 * PANEL_SIZE, y as f32 * PANEL_SIZE, 1.),
            rotation: Quat::from_rotation_x(-30.),
            ..Default::default()
        },
        ..Default::default()
    });
}

// positionをtranslationに変換
fn position_to_translation_system(mut position_query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in position_query.iter_mut() {
        transform.translation.y = position.y;
        transform.translation.x = position.x;
        transform.translation.z = if position.visible {
            VISIBLE_OBJECT_Z
        } else {
            INVISIBLE_OBJECT_Z
        };
    }
}
