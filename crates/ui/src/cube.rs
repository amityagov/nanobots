use crate::cube_mesh::create_cube_mesh;
use bevy::pbr::experimental::meshlet::{
    MaterialMeshletMeshBundle, Meshlet, MeshletMesh, MeshletPlugin,
};
use bevy::prelude::*;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshletPlugin);
        app.add_systems(Startup, setup)
            .add_systems(Update, render_cube);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut meshlets: ResMut<Assets<MeshletMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = create_cube_mesh();

    let meshlet = MeshletMesh::from_mesh(&mesh).unwrap();
    let meshlet = meshlets.add(meshlet);
    let mesh = meshes.add(Cuboid::new(0.98, 0.98, 0.98));
    let material = materials.add(StandardMaterial {
        perceptual_roughness: 0.09,
        base_color: Color::srgb(0.1, 0.1, 1.0),
        ..Default::default()
    });

    commands.spawn(CubeTemplate(mesh, meshlet, material));
}

#[derive(Component)]
struct CubeTemplate(Handle<Mesh>, Handle<MeshletMesh>, Handle<StandardMaterial>);

#[derive(Component)]
pub struct Cube(pub usize, pub usize, pub usize);

fn render_cube(
    mut commands: Commands,
    mut template: Query<(Entity, &CubeTemplate)>,
    mut query: Query<(Entity, &mut Cube), Added<Cube>>,
) {
    let template = template.single().1;
    for (entity, cube) in query.iter() {
        commands.spawn(MaterialMeshletMeshBundle {
            meshlet_mesh: template.1.clone(),
            material: template.2.clone(),
            transform: Transform::from_xyz(
                1.0 * cube.0 as f32,
                1.0 * cube.1 as f32,
                1.0 * cube.2 as f32,
            ),
            ..Default::default()
        });
        commands.entity(entity).despawn();
    }
}
