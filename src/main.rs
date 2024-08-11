#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
use bevy::prelude::*;
use spacerlite::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}
