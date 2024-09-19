mod camera;
mod model;

use crate::camera::CameraPlugin;
use crate::model::{LoadModelEvent, ModelPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiContexts, EguiPlugin};
use std::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Model Viewer".to_owned(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            EguiPlugin,
        ))
        .add_plugins((CameraPlugin, ModelPlugin))
        .add_systems(Update, ui_system)
        .add_systems(Startup, render_cube)
        .add_systems(Update, render_gizmos)
        .run();
}

#[derive(Component)]
struct CubeTemplate(Handle<Mesh>, Handle<StandardMaterial>);

fn render_gizmos(mut gizmos: Gizmos) {
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(40),
        Vec2::new(1., 1.),
        LinearRgba::WHITE,
    );
    gizmos.grid(
        Vec3::new(0.0, 20.0, 20.0),
        Quat::from_rotation_y(PI / 2.),
        UVec2::splat(40),
        Vec2::new(1., 1.),
        LinearRgba::gray(0.5),
    );
    gizmos.grid(
        Vec3::new(20.0, 0.0, 20.0),
        Quat::from_rotation_z(PI / 2.),
        UVec2::splat(40),
        Vec2::new(1., 1.),
        LinearRgba::WHITE,
    );
}

fn render_cube(
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    ambient_light.brightness = 200.0;
    ambient_light.color = Color::WHITE;

    let mesh = meshes.add(Cuboid::new(0.98, 0.98, 0.98));
    let mat = materials.add(StandardMaterial {
        perceptual_roughness: 0.09,
        base_color: Color::srgb(0.1, 0.1, 1.0),
        ..Default::default()
    });

    commands.spawn(CubeTemplate(mesh.clone(), mat.clone()));

    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                commands.spawn(PbrBundle {
                    mesh: mesh.clone_weak(),
                    material: mat.clone_weak(),
                    transform: Transform::from_xyz(1.0 * x as f32, 1.0 * y as f32, 1.0 * z as f32),
                    ..Default::default()
                });
            }
        }
    }
}

fn ui_system(mut contexts: EguiContexts, mut ev_load_model: EventWriter<LoadModelEvent>) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });

            egui::menu::menu_button(ui, "Model", |ui| {
                if ui.button("Load model").clicked() {
                    ev_load_model.send(LoadModelEvent);
                }
            });
        });
    });
}
