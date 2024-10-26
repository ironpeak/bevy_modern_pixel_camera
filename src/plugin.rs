use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostUpdate};
use bevy::render::camera::{self, OrthographicProjection};

/// Provides the camera system.
pub struct PixelCameraPlugin;

#[allow(deprecated)]
impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            crate::zoom::pixel_zoom_system.after(camera::camera_system::<OrthographicProjection>),
        );
    }
}
