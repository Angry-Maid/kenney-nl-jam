use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::types::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Splash,
    CameraIcon,
    UiButton,
    UiHeader,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::Splash,
                asset_server.load_with_settings(
                    "images/splash.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::UiButton,
                asset_server.load_with_settings(
                    "images/ui/button_rectangle_depth_gradient.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::CameraIcon,
                asset_server.load("images/camera_icon.png"),
            ),
            (
                ImageKey::UiHeader,
                asset_server.load_with_settings(
                    "images/ui/button_rectangle_line.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
        ]
        .into()
    }
}
