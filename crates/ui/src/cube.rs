use crate::instancing::{InstanceData, InstanceMaterialData};
use bevy::prelude::*;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_cube);
    }
}

#[derive(Component)]
pub struct Cube(pub usize, pub usize, pub usize);

fn render_cube(
    mut commands: Commands,
    query: Query<(Entity, &Cube), Added<Cube>>,
    mut instancing: Query<(Entity, &mut InstanceMaterialData)>,
) {
    let mut instance = instancing.get_single_mut().unwrap();

    for (entity, cube) in query.iter() {
        instance.1.push(InstanceData {
            position: Vec3::new(
                1.0 * cube.0 as f32,
                1.0 * cube.1 as f32,
                1.0 * cube.2 as f32,
            ),
            scale: 1.0,
            color: LinearRgba::from(Color::srgb(0.2, 0.4, 0.8)).to_f32_array(),
        });

        commands.entity(entity).despawn();
    }
}
