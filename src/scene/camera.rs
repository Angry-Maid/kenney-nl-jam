use bevy::{
    core::Name,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{component::Component, system::Commands},
    math::Vec3,
    render::camera::{OrthographicProjection, ScalingMode},
    transform::components::Transform,
    ui::IsDefaultUiCamera,
};

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            projection: OrthographicProjection {
                // For this example, let's make the screen/window height
                // correspond to 16.0 world units.
                scaling_mode: ScalingMode::FixedVertical(16.0),
                ..Default::default()
            }
            .into(),
            // NOTE:
            // Distance might affect shadows and clipping / culling)
            // TODO:
            // Provide a Quaternion instead. Better for relocating the camera and maintaining a set
            // orientation.
            transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
