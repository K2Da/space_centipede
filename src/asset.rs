use crate::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::random;
use std::f32::consts::PI;

pub struct ModPlugin {}
impl Plugin for ModPlugin {
    fn build(&self, _app: &mut App) {}
}
#[derive(Resource)]
pub struct Handles {
    gate_poll_mesh: Mesh2dHandle,
    gate_poll_material: Handle<ColorMaterial>,
    head_mesh: Mesh2dHandle,
    head_material: Handle<ColorMaterial>,
    center_marker_mesh: Mesh2dHandle,
    center_marker_material: Handle<ColorMaterial>,
}

impl Handles {
    pub fn head_mesh_bundle(&self) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: self.head_mesh.clone(),
            material: self.head_material.clone(),
            ..default()
        }
    }
    pub fn center_marker_mesh_bundle(&self) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: self.center_marker_mesh.clone(),
            material: self.center_marker_material.clone(),
            ..default()
        }
    }
    pub fn gate_poll_mesh_bundle(&self, length: f32) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: self.gate_poll_mesh.clone(),
            material: self.gate_poll_material.clone(),
            transform: Transform::from_translation(Vec3::new(length / 2.0, 0.0, 1.0)),
            ..default()
        }
    }

    pub fn chain_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(2.0, 20.0)),
                ..default()
            },
            ..default()
        }
    }

    pub fn gate_bundle(&self, length: f32) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: BAR_COLOR,
                custom_size: Some(Vec2::new(length, 2.0)),
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(random::<f32>() * PI),
                ..default()
            },
            ..default()
        }
    }
}

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Handles {
        gate_poll_mesh: meshes.add(Circle::new(POLL_SIZE)).into(),
        gate_poll_material: materials.add(ColorMaterial::from(POLL_COLOR)),
        head_mesh: meshes.add(Circle::new(HEAD_SIZE)).into(),
        head_material: materials.add(ColorMaterial::from(HEAD_COLOR)),
        center_marker_mesh: meshes.add(Circle::new(3.0)).into(),
        center_marker_material: materials.add(ColorMaterial::from(Color::PURPLE)),
    });
}
