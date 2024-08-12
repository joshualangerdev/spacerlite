use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::palette::LABEL_TEXT;

pub trait Containers {
    fn ui_root(&mut self) -> EntityCommands;
}

pub trait Widgets {
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..Default::default()
            },
        ))
    }
}

impl<T: Spawn> Widgets for T {
    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Label"),
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.0,
                    color: LABEL_TEXT,
                    ..default()
                },
            )
            .with_style(Style {
                width: Px(500.0),
                ..Default::default()
            }),
        ));
        entity
    }
}

trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
