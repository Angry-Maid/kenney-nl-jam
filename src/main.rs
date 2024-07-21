// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
#![feature(lazy_cell)]

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    log::LogPlugin,
    prelude::*,
};

mod asset_management;
mod config;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod scene;
mod screen;
mod ui;
mod util;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn main() -> AppExit {
    let mut app = App::new();
    app
        // Bevy-Native Plugins
        .add_plugins({
            // NOTE:
            // I did this so that I could set `LogPlugin` for `dev` feature. It was not
            // obviously how to modify a plugin retroactively (after it had already been added to
            // the App).
            let default_plugins = DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Kenney Nl Jam".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.25),
                    },
                    ..default()
                });

            #[cfg(feature = "dev")]
            let default_plugins = default_plugins.set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,kenney-nl-jam=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            });

            default_plugins
        })
        .add_plugins((
            game::plugin,
            screen::plugin,
            ui::plugin,
            asset_management::plugin,
        ))
        .configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

    // Enable dev tools for dev builds.
    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.run()
}
