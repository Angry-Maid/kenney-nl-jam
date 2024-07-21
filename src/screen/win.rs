use bevy::prelude::*;

use super::Screen;
use crate::{
    asset_management::{fonts::FontKey, images::ImageKey, types::HandleMap},
    ui::interaction::InteractionQuery,
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Win), enter_win);
    app.add_systems(OnExit(Screen::Win), exit_win);

    app.add_systems(Update, handle_finish_action.run_if(in_state(Screen::Win)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum WinAction {
    Back,
}

fn enter_win(
    mut commands: Commands,
    font_handles: Res<HandleMap<FontKey>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Win))
        .with_children(|children| {
            children.header(
                "You Won",
                Some(image_handles[&ImageKey::UiHeader].clone_weak()),
            );

            children
                .button(
                    "",
                    Some(font_handles[&FontKey::FontAwesome].clone_weak()),
                    Some(image_handles[&ImageKey::UiButton].clone_weak()),
                )
                .insert(WinAction::Back);
        });
}

fn exit_win(mut commands: Commands) {}

fn handle_finish_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&WinAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                WinAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
