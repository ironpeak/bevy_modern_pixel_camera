use bevy::{
    prelude::*,
    render::camera::{CameraProjection, NormalizedRenderTarget, ScalingMode},
    utils::HashSet,
    window::{PrimaryWindow, WindowCreated, WindowResized, WindowScaleFactorChanged},
};

// from: https://github.com/drakmaniso/bevy_pixel_camera/blob/main/src/pixel_zoom.rs
#[derive(Component, Debug, Clone, PartialEq)]
/// Configure a `Camera2dBundle` to use integer scaling and automatically match
/// a specified resolution.
///
/// Note: when this component is present, a plugin system will automatically
/// update the `ScalingMode` of the camera bundle.
pub enum PixelZoom {
    /// Manually specify the camera zoom, i.e. the number of screen pixels
    /// (logical pixels) used to display one virtual pixel (world unit).
    Fixed(i32),
    /// Automatically set the camera zoom to fit the specified resolution inside
    /// the window.
    FitSize { width: i32, height: i32 },
    /// Automatically set the camera zoom to fit the specified width inside the
    /// window.
    FitWidth(i32),
    /// Automatically set the camera zoom to fit the specified height inside the
    /// window.
    FitHeight(i32),
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn pixel_zoom_system<T: CameraProjection + Component>(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    mut window_scale_factor_changed_events: EventReader<WindowScaleFactorChanged>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    mut cameras: Query<(&mut Camera, &PixelZoom, &mut OrthographicProjection)>,
) {
    // from: https://github.com/bevyengine/bevy/blob/release-0.14.2/crates/bevy_render/src/camera/camera.rs
    let primary_window = primary_window.iter().next();

    let mut changed_window_ids = HashSet::new();
    changed_window_ids.extend(window_created_events.read().map(|event| event.window));
    changed_window_ids.extend(window_resized_events.read().map(|event| event.window));
    let scale_factor_changed_window_ids: HashSet<_> = window_scale_factor_changed_events
        .read()
        .map(|event| event.window)
        .collect();
    changed_window_ids.extend(scale_factor_changed_window_ids.clone());

    let changed_image_handles: HashSet<&AssetId<Image>> = image_asset_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } | AssetEvent::Added { id } => Some(id),
            _ => None,
        })
        .collect();

    for (camera, pixel_zoom, mut projection) in &mut cameras {
        if let Some(normalized_target) = camera.target.normalize(primary_window) {
            if is_changed(
                &normalized_target,
                &changed_window_ids,
                &changed_image_handles,
            ) || camera.is_added()
                || projection.is_changed()
            {
                // from: https://github.com/drakmaniso/bevy_pixel_camera/blob/main/src/pixel_zoom.rs
                let logical_size = match camera.logical_target_size() {
                    Some(size) => size,
                    None => continue,
                };

                let zoom = auto_zoom(pixel_zoom, logical_size) as f32;
                match projection.scaling_mode {
                    ScalingMode::WindowSize(previous_zoom) => {
                        if previous_zoom != zoom {
                            projection.scaling_mode = ScalingMode::WindowSize(zoom)
                        }
                    }
                    _ => projection.scaling_mode = ScalingMode::WindowSize(zoom),
                }
            }
        }
    }
}

fn is_changed(
    normalized_target: &NormalizedRenderTarget,
    changed_window_ids: &HashSet<Entity>,
    changed_image_handles: &HashSet<&AssetId<Image>>,
) -> bool {
    // from: https://github.com/bevyengine/bevy/blob/release-0.14.2/crates/bevy_render/src/camera/camera.rs
    match normalized_target {
        NormalizedRenderTarget::Window(window_ref) => {
            changed_window_ids.contains(&window_ref.entity())
        }
        NormalizedRenderTarget::Image(image_handle) => {
            changed_image_handles.contains(&image_handle.id())
        }
        NormalizedRenderTarget::TextureView(_) => true,
    }
}

fn auto_zoom(mode: &PixelZoom, logical_size: Vec2) -> i32 {
    // from: https://github.com/drakmaniso/bevy_pixel_camera/blob/main/src/pixel_zoom.rs
    match mode {
        PixelZoom::FitSize { width, height } => {
            let zoom_x = (logical_size.x as i32) / i32::max(*width, 1);
            let zoom_y = (logical_size.y as i32) / i32::max(*height, 1);
            let zoom = i32::min(zoom_x, zoom_y);
            i32::max(zoom, 1)
        }
        PixelZoom::FitWidth(width) => {
            let zoom = (logical_size.x as i32) / i32::max(*width, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::FitHeight(height) => {
            let zoom = (logical_size.y as i32) / i32::max(*height, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::Fixed(zoom) => *zoom,
    }
}
