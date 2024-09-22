use crate::cube::Cube;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use mdl::{Cell, CellState, Matrix};
use rfd::FileDialog;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Event)]
pub struct LoadModelEvent {
    pub file: Option<PathBuf>,
}

#[derive(Event, Debug)]
pub struct RenderModelEvent;

#[derive(Event, Debug)]
pub struct ClearModelEvent;

#[derive(Component)]
struct ModelStatsText;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadModelEvent>();
        app.add_event::<RenderModelEvent>();
        app.add_event::<ClearModelEvent>();

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                listen_load_model_events,
                listen_render_model_events,
                listen_clear_model_events,
                poll_model_select,
                render_model,
            ),
        );
    }
}

pub struct ModelData {
    path: String,
    matrix: Matrix,
    elapsed: Duration,
}

#[derive(Component)]
struct ModelRenderProgress {
    matrix: Matrix,
    level: usize,
    started: bool,
    count: usize,
}

#[derive(Component, Default)]
pub struct SelectedModelState {
    pub data: Option<ModelData>,
}

#[derive(Component)]
struct ModelFileSelectionTask(Task<Option<ModelData>>);

async fn load_matrix(path: Option<PathBuf>) -> anyhow::Result<ModelData> {
    let filename = path.or_else(|| {
        FileDialog::new()
            .add_filter("Model file", &["mdl"])
            .pick_file()
    });

    if let Some(filename) = filename {
        let start = Instant::now();
        let file = File::open(&filename)?;
        let mut reader = BufReader::new(file);
        let matrix = mdl::read_matrix(&mut reader)?;
        let elapsed = start.elapsed();
        return Ok(ModelData {
            matrix: matrix,
            path: filename.to_string_lossy().to_string(),
            elapsed,
        });
    }

    Err(anyhow::anyhow!("No valid filename"))
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SelectedModelState::default());

    commands.spawn((
        ModelStatsText,
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    ..default()
                },
            ),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(28.0),
            left: Val::Px(15.0),
            ..default()
        }),
    ));
}

fn render_model(mut model: Query<(Entity, &mut ModelRenderProgress)>, mut commands: Commands) {
    if let Ok((entity, mut data)) = model.get_single_mut() {
        if !data.started {
            data.started = true;
        }
        let cubes = data
            .matrix
            .get_level(data.level)
            .into_iter()
            .filter(|x| x.state == CellState::Fill)
            .collect::<Vec<Cell>>();

        if cubes.len() > 0 {
            data.level += 1;
            for x in cubes {
                commands.spawn(Cube(x.x, x.y, x.z));
                data.count += 1;
            }
        } else {
            commands.entity(entity).despawn();
            println!("{}", data.count);
        }
    }
}

fn listen_load_model_events(mut commands: Commands, mut events: EventReader<LoadModelEvent>) {
    events.read().for_each(|event| {
        let thread_pool = AsyncComputeTaskPool::get();
        let path = event.file.clone();
        let task = thread_pool.spawn(async move { load_matrix(path).await.ok() });
        commands.spawn(ModelFileSelectionTask(task));
    });
}

fn listen_render_model_events(
    mut commands: Commands,
    mut events: EventReader<RenderModelEvent>,
    model: Query<(Entity, &SelectedModelState)>,
) {
    events.read().for_each(|event| {
        if let Ok(model) = model.get_single().map(|x| x.1) {
            if let Some(data) = &model.data {
                commands.spawn(ModelRenderProgress {
                    matrix: data.matrix.clone(),
                    level: 0,
                    started: false,
                    count: 0,
                });
            }
        }
    });
}

fn listen_clear_model_events(mut _commands: Commands, mut _events: EventReader<ClearModelEvent>) {
    // events.read().for_each(|event| {});
}

fn poll_model_select(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ModelFileSelectionTask)>,
    mut model: Query<(Entity, &mut SelectedModelState)>,
    mut model_stat_text: Query<&mut Text, With<ModelStatsText>>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            if let Some(data) = result {
                commands.entity(entity).remove::<ModelFileSelectionTask>();
                for mut text in &mut model_stat_text {
                    text.sections[0].value = format!(
                        "Model {}, R: {}, loaded in {:?}",
                        data.path, data.matrix.r, data.elapsed
                    );
                }

                if let Ok(mut model) = model.get_single_mut() {
                    model.1.data = Some(data);
                }
            }
        }
    }
}
