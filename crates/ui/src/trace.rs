use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use rfd::FileDialog;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[derive(Event)]
pub struct LoadTraceEvent;

#[derive(Component)]
struct TraceFileSelectionTask(Task<Option<TraceData>>);

struct TraceData {
    path: String,
    commands: Vec<nbt::Command>,
    elapsed: Duration,
}

pub struct TracePlugin;

impl Plugin for TracePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadTraceEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, (listen_load_trace_events, poll_model_select));
    }
}

#[derive(Component)]
struct TraceStatsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TraceStatsText,
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
            bottom: Val::Px(28.0 * 1.7),
            left: Val::Px(15.0),
            ..default()
        }),
    ));
}

async fn load_trace() -> anyhow::Result<TraceData> {
    let filename = FileDialog::new()
        .add_filter("Trace file", &["nbt"])
        .pick_file();

    if let Some(filename) = filename {
        let start = Instant::now();
        let file = File::open(&filename)?;
        let mut reader = BufReader::new(file);
        let commands = nbt::read_commands(&mut reader)?;
        let elapsed = start.elapsed();
        return Ok(TraceData {
            commands,
            path: filename.to_string_lossy().to_string(),
            elapsed,
        });
    }

    Err(anyhow::anyhow!("No valid filename"))
}

fn listen_load_trace_events(mut commands: Commands, mut events: EventReader<LoadTraceEvent>) {
    events.read().for_each(|_| {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move { load_trace().await.ok() });
        commands.spawn(TraceFileSelectionTask(task));
    });
}

fn poll_model_select(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut TraceFileSelectionTask)>,
    mut model_stat_text: Query<&mut Text, With<TraceStatsText>>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            if let Some(data) = result {
                commands.entity(entity).remove::<TraceFileSelectionTask>();
                for mut text in &mut model_stat_text {
                    text.sections[0].value = format!(
                        "Trace {} with commands: {}, loaded in {:?}",
                        data.path,
                        data.commands.len(),
                        data.elapsed
                    );
                }
            }
        }
    }
}
