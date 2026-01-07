use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const SHADER: &str = "shaders/water.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<WaterMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(WaterMaterial {
            color: LinearRgba::BLUE,
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
}
