use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use rfd::FileDialog;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[derive(Event)]
pub struct LoadModelEvent;

#[derive(Component)]
struct ModelStatsText;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadModelEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, (listen_load_model_events, poll_model_select));
    }
}

struct ModelData {
    path: String,
    matrix: mdl::Matrix,
    elapsed: Duration,
}

#[derive(Component)]
struct ModelFileSelectionTask(Task<Option<ModelData>>);

async fn load_matrix() -> anyhow::Result<ModelData> {
    let filename = FileDialog::new()
        .add_filter("Model file", &["mdl"])
        .pick_file();

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

fn listen_load_model_events(mut commands: Commands, mut events: EventReader<LoadModelEvent>) {
    events.read().for_each(|_| {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move { load_matrix().await.ok() });
        commands.spawn(ModelFileSelectionTask(task));
    });
}

fn poll_model_select(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ModelFileSelectionTask)>,
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
            }
        }
    }
}
