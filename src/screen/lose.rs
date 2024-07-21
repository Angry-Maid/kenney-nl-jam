use bevy::prelude::*;

use super::Screen;
use crate::{
    asset_management::{fonts::FontKey, images::ImageKey, types::HandleMap},
    ui::interaction::InteractionQuery,
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Lose), enter_lose);
    app.add_systems(OnExit(Screen::Lose), exit_lose);

    app.add_systems(Update, handle_finish_action.run_if(in_state(Screen::Lose)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum LoseAction {
    Back,
}

fn enter_lose(
    mut commands: Commands,
    font_handles: Res<HandleMap<FontKey>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Lose))
        .with_children(|children| {
            children.header(
                "You Lost",
                Some(image_handles[&ImageKey::UiHeader].clone_weak()),
            );

            children
                .button(
                    "",
                    Some(font_handles[&FontKey::FontAwesome].clone_weak()),
                    Some(image_handles[&ImageKey::UiButton].clone_weak()),
                )
                .insert(LoseAction::Back);
        });
}

fn exit_lose(mut commands: Commands) {}

fn handle_finish_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&LoseAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                LoseAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
