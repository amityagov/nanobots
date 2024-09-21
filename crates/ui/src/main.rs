mod camera;
mod cube;
mod cube_mesh;
mod instancing;
mod model;
mod trace;

use crate::camera::CameraPlugin;
use crate::cube::CubePlugin;
use crate::instancing::InstancingPlugin;
use crate::model::{LoadModelEvent, ModelPlugin, RenderModelEvent, SelectedModelState};
use crate::trace::{LoadTraceEvent, TracePlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_obj::ObjPlugin;
use bevy_stl::StlPlugin;
use egui::Frame;
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
        .add_plugins((
            CameraPlugin,
            ModelPlugin,
            TracePlugin,
            CubePlugin,
            StlPlugin,
            ObjPlugin,
            InstancingPlugin,
        ))
        .add_systems(Update, ui_system)
        .add_systems(Startup, render_cube)
        .add_systems(Update, render_gizmos)
        .run();
}

fn render_gizmos(mut gizmos: Gizmos) {
    return;
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

fn render_cube(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 200.0;
    ambient_light.color = Color::WHITE;
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ev_load_model: EventWriter<LoadModelEvent>,
    mut ev_load_trace: EventWriter<LoadTraceEvent>,
    mut ev_render_trace: EventWriter<RenderModelEvent>,
    model: Query<(Entity, &SelectedModelState)>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::bottom("text").show(ctx, |ui| {
        Frame::default().inner_margin(2.0).show(ui, |ui| {
            ui.horizontal(|ui| {
                let has_model = model
                    .get_single()
                    .map_or_else(|_| false, |x| x.1.data.is_some());

                if ui
                    .add_enabled(has_model, egui::Button::new("Render model"))
                    .clicked()
                {
                    ev_render_trace.send(RenderModelEvent);
                }

                if ui
                    .add_enabled(has_model, egui::Button::new("Clear model"))
                    .clicked()
                {
                    // clear
                }

                ui.separator();

                if ui.button("Run trace").clicked() {
                    println!("Run trace");
                }

                if ui.button("Clear trace").clicked() {
                    println!("clicked");
                }
            });
        });
    });

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

            egui::menu::menu_button(ui, "Trace", |ui| {
                if ui.button("Load trace").clicked() {
                    ev_load_trace.send(LoadTraceEvent);
                }
            });
        });
    });
}
