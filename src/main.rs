use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

enum Chirality {
    Left,
    Right,
    Foo,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = Vec::from([
        // ----
        // ----
        // ----
        // ----
        // ----
        // ----
        Vec3::new(1.0, 0.0, 0.0),
    ]);

    // You are given two points.
    // v0 is to your left and v1 is to your right.
    let mut v0 = Vec3::new(0.0, 0.5, 0.0);
    let mut v1 = Vec3::new(0.0, -0.5, 0.0);

    // This will be the 3rd point of our first triangle.
    // Its initial value is inconsequential.
    let mut v2 = Vec3::ZERO;

    // You are told that +Z is your up, therefore
    // +X is in front of you.
    let mut up = Vec3::Z;
    let mut right = -Vec3::Y;

    // Don't look at this.
    let mut new_up = Vec3::ZERO;

    // We will place a visual point at these points.
    let mut vertices = vec![v0, v1];
    // And will display our edges too.
    let mut edges = vec![(Chirality::Foo, v0, v1)];

    for (i, point) in points.iter().enumerate() {
        let origin = (v0 + v1) / 2.0;
        let new_right = (v0 - v1).normalize();

        println!(
            "\n\n---------------------[ points[{}] = {} ]---------------------",
            i, point
        );
        println!("origin={}", origin);
        println!("v0={}", v0);
        println!("v1={}", v1);
        println!("v2={} [!from last pass!]", v2);
        println!("---");
        println!("right={}", right);
        println!("new_right={}", new_right);
        println!("up={}", up);

        let mut transform = Transform::from_translation(*point + origin);
        transform.rotate(Quat::from_rotation_arc(up, new_up));
        transform.rotate(Quat::from_rotation_arc(right, new_right));

        v2 = transform.translation;

        new_up = (v0 - v1).cross(v1 - v2).normalize();
        println!("new_up={}", new_up);

        vertices.push(v2);
        edges.push((Chirality::Left, v0, v2));
        edges.push((Chirality::Right, v1, v2));

        v1 = v2;
        up = new_up;
        right = new_right;
        println!("\n\n\n");
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0 / 10.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        ..default()
    });

    for vertex in vertices {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0 / 20.0,
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
    for (chirality, a, b) in edges {
        let height = (a - b).length();
        let direction = (a - b).normalize();
        let base_color = match chirality {
            Chirality::Foo => Color::WHITE,
            Chirality::Left => Color::BLUE,
            Chirality::Right => Color::RED,
        };
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 1.0 / 30.0,
                height,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color,
                ..default()
            }),
            transform: Transform::from_translation((a + b) / 2.0)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, direction)),
            ..default()
        });
    }

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
