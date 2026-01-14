// from: https://github.com/drakmaniso/bevy_pixel_camera/blob/main/src/pixel_zoom.rs
use bevy::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera that will always fit the virtual resolution WIDTH x HEIGHT
    // inside the window.
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        WithUiScaling,
        PixelZoom::FitSize {
            width: WIDTH,
            height: HEIGHT,
        },
        PixelViewport,
    ));

    // Add a Flat button
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            border_radius: BorderRadius::MAX,
            row_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                base_button("Button 1"),
                BorderColor::all(Color::BLACK),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ),
            (
                base_button("Button 2"),
                ImageNode {
                    image: asset_server.load("button.png"),
                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                        border: BorderRect {
                            min_inset: Vec2::splat(4.),
                            max_inset: Vec2::new(4., 5.)
                        },
                        ..default()
                    }),
                    ..default()
                },
            ),
        ],
    ));
}

fn base_button(text: impl Into<String>) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(60.0),
            height: Val::Px(20.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont::from_font_size(8.0),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}
