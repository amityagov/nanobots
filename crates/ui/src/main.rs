use std::path::PathBuf;
use bevy::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::window::WindowResolution;
use bevy_egui::{EguiContexts, EguiPlugin};
use rfd::FileDialog;
use futures_lite::future;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            EguiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .add_systems(Update, poll_model_select_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SelectedFile(Task<Option<PathBuf>>);

fn ui_system(mut contexts: EguiContexts, mut commands: Commands) {
    egui::Window::new("").show(contexts.ctx_mut(), |ui| {
        // load model button
        if ui.button("Load model").clicked() {
            let thread_pool = AsyncComputeTaskPool::get();
            let task = thread_pool.spawn(async move {
                FileDialog::new()
                    .add_filter("Model file             ", &["mdl"])
                    .pick_file()
            });
            commands.spawn(SelectedFile(task));
        }
    });
}

// Poll model file loading
fn poll_model_select_system(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFile)>
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(
            future::poll_once(&mut selected_file.0)
        ) {
            info!("Selected file {:?}", result);
            commands.entity(entity).remove::<SelectedFile>();
        }
    }
}