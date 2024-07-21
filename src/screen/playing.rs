//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{
    asset_management::audio::SoundtrackKey, game::audio::soundtrack::PlaySoundtrack, scene,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(scene::plugin);

    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Backspace))),
    );
    app.add_systems(
        Update,
        to_lose_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::KeyO))),
    );
    app.add_systems(
        Update,
        to_win_screen.run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::KeyP))),
    );
}

fn enter_playing(mut commands: Commands) {
    // commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    // commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn to_lose_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Lose);
}

fn to_win_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Win);
}
