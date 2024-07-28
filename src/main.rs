use bevy::{prelude::*, DefaultPlugins};

mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_player, systems::camera::spawn_camera, systems::level::spawn_floor, spawn_light),
        )
        .run();
}



fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(player);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}