//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{
    assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_level);
    app.add_systems(OnExit(Screen::Playing), disable_soundtrack);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn disable_soundtrack(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entities instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
