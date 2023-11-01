use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = Vec::from([
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ]);

    let v0 = Vec3::new(0.0, -0.5, 0.0); // mut for right only!!
    let mut v1 = Vec3::new(0.0, 0.5, 0.0);
    let mut v2 = Vec3::new(1.0, 0.0, 0.0); // Dummy point to establish "up"

    // Standing on the XY plane, Z is "up" and X is "forward"
    //let mut origin = Vec3::ZERO;
    let mut up = Vec3::Z;
    let mut right = -Vec3::Y;

    let mut vertices = vec![v0, v1];
    let mut edges = vec![(v0, v1)];

    for point in points {
        let origin = (v0 + v1) / 2.0;
        let new_right = (v0 - v1).normalize();
        let new_up = (v1 - v2).cross(v0 - v1).normalize();

        let mut transform = Transform::from_translation(point + origin);
        transform.rotate(Quat::from_rotation_arc(up, new_up));
        transform.rotate(Quat::from_rotation_arc(right, new_right));

        v2 = transform.translation;

        vertices.push(v2);
        edges.push((v0, v2));
        edges.push((v1, v2));

        v1 = v2;
        up = new_up;
        right = new_right;

        /*

           transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
           transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
           transform.rotate(Quat::from_axis_angle(local_y, rotation.y));

        */
    }

    for vertex in vertices {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0 / 40.0,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            transform: Transform::from_translation(vertex),
            ..default()
        });
    }
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
