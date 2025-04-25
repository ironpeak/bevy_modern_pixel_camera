use bevy::{
    ecs::schedule::IntoScheduleConfigs,
    prelude::{App, Plugin, PostUpdate},
    render::camera::{self},
};

/// Provides the camera system.
pub struct PixelCameraPlugin;

#[allow(deprecated)]
impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            crate::zoom::pixel_zoom_system.after(camera::camera_system),
        );
    }
}
