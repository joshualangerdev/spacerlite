mod splash;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins(splash::plugin);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    //Title,
    //Game,
}
