use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::screens::Screen;

const DEBUG_MODE_KEY: KeyCode = KeyCode::Backquote;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins(DebugUiPlugin);
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(DEBUG_MODE_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
