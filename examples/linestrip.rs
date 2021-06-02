use bevy::{pbr::PointLightBundle, prelude::*};
use bevy_polyline::{
    pipeline::{new_polyline_pbr_pipeline, new_polyline_pipeline},
    Polyline, PolylineBundle, PolylineMaterial, PolylinePbrBundle, PolylinePbrMaterial,
    PolylinePlugin,
};

fn main() {
    let mut app = App::build();

    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PolylinePlugin)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut polyline_pbr_materials: ResMut<Assets<PolylinePbrMaterial>>,
) {
    commands.spawn_bundle(PolylinePbrBundle {
        polyline: Polyline {
            vertices: vec![
                Vec3::new(-0.5, 0.0, -0.5),
                Vec3::new(0.5, 0.0, -0.5),
                Vec3::new(0.5, 1.0, -0.5),
                Vec3::new(-0.5, 1.0, -0.5),
                Vec3::new(-0.5, 1.0, 0.5),
                Vec3::new(0.5, 1.0, 0.5),
                Vec3::new(0.5, 0.0, 0.5),
                Vec3::new(-0.5, 0.0, 0.5),
            ],
            ..Default::default()
        },
        render_pipelines: RenderPipelines {
            pipelines: vec![new_polyline_pbr_pipeline(true)],
            ..Default::default()
        },
        polyline_pbr_material: polyline_pbr_materials.add(PolylinePbrMaterial {
            width: 15.0,
            perspective: true,
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: standard_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: standard_materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..PerspectiveCameraBundle::new_3d()
        })
        .insert(Rotates);
}

/// this component indicates what entities should rotate
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}
