# bevy_modern_pixel_camera

Based of: https://github.com/drakmaniso/bevy_pixel_camera

A simple camera plugin for the Bevy game engine, to help with the use of
pixel-art sprites.

This crates provides a plugin to automatically configure Bevy's
`Camera2dBundle`. It works by setting the camera to an integer scaling
factor (using Bevy's `ScalingMode::WindowSize`), and automatically updating
the zoom level so that the specified target resolution fills as much of the
sceen as possible.

The plugin can also automatically set and resize the viewport of the camera
to match the target resolution.

## How to use

Note that Bevy uses linear sampling by default for textures, which is not
what you want for pixel art. The easiest way to change this is to configure
Bevy's default plugins with `ImagePlugin::default_nearest()`.

Note that Bevy uses [MSAA](https://en.wikipedia.org/wiki/Multisample_anti-aliasing) 
by default for textures, which is not what you want for pixel art The easiest way 
to change this is to configure Bevy's by overriding the Msaa resource with 
`.insert_resource(Msaa::Off)`.

Also note that if either the width or the height of your sprite is not
divisible by 2, you may need to change the anchor of the sprite (which is at
the center by default), otherwise it won't be aligned with virtual pixels.

```rust
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("my-pixel-art-sprite.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}
```

A small example is included in the crate. Run it with:

```console
cargo run --example mire
```

## Bevy versions supported

| bevy | bevy_modern_pixel_camera |
| ---- | ------------------------ |
| 0.15 | 0.2                      |
| 0.14 | 0.1                      |

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

License: MIT OR Apache-2.0