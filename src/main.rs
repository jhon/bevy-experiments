use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .run()
}

fn setup(mut commands: Commands) {
    // let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.projection = OrthographicProjection { scale: };
    // // camera_bundle.orthographic_projection.scale = 1. / 50.;
    // commands.spawn_bundle(camera_bundle);

    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            scaling_mode: ScalingMode::FixedVertical(24.0),
            ..default()
        }.into(),
        ..default()
    });
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
}

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.04;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}
