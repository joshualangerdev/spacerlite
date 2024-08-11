mod screens;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameSet::TickTimers, GameSet::RecordInput, GameSet::Update).chain(),
        );

        app.add_systems(Startup, spawn_camera);

        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Spacerlite".to_string(),
                        canvas: Some("#space".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..Default::default()
                }),
        );
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum GameSet {
    TickTimers,
    RecordInput,
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}
