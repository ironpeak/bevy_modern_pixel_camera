// from: https://github.com/drakmaniso/bevy_pixel_camera/blob/main/src/pixel_zoom.rs
use bevy::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;
const SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Moving {
    direction: Vec2,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera that will always fit the virtual resolution WIDTH x HEIGHT
    // inside the window.
    commands.spawn((
        Camera2d::default(),
        PixelZoom::FitSize {
            width: WIDTH,
            height: HEIGHT,
        },
        PixelViewport,
    ));

    let mire_handle = asset_server.load("mire-64x64.png");

    // Add a mire sprite in the center of the window.
    commands.spawn((
        Moving {
            direction: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                .normalize(),
        },
        Sprite {
            image: mire_handle.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
    ));

    // Add a mire sprite in the bottom-left corner of our virtual resolution.
    commands.spawn((
        Sprite {
            image: mire_handle.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(-(WIDTH / 2) as f32, -(HEIGHT / 2) as f32, 0.0)),
    ));

    // Add a mire sprite in the bottom-right corner of our virtual resolution.
    commands.spawn((
        Sprite {
            image: mire_handle.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new((WIDTH / 2) as f32, -(HEIGHT / 2) as f32, 0.0)),
    ));

    // Add a mire sprite in the top-left corner of our virtual resolution.
    commands.spawn((
        Sprite {
            image: mire_handle.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(-(WIDTH / 2) as f32, (HEIGHT / 2) as f32, 0.0)),
    ));

    // Add a mire sprite in the top-right corner of our virtual resolution.
    commands.spawn((
        Sprite {
            image: mire_handle.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32, 0.0)),
    ));
}

fn update(mut query: Query<(&mut Transform, &mut Moving)>, time: Res<Time>) {
    for (mut transform, mut moving) in query.iter_mut() {
        transform.translation += moving.direction.extend(0.0) * SPEED * time.delta_secs();
        if transform.translation.x.abs() > WIDTH as f32 / 2.0 {
            moving.direction.x *= -1.0;
        }
        if transform.translation.y.abs() > HEIGHT as f32 / 2.0 {
            moving.direction.y *= -1.0;
        }
    }
}
