use bevy::prelude::*;
use crate::{AppState, spawn_back_button};

pub fn spawn_vocabulary_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(30.0), // Adds space between the two sections
                    ..default()
        },
        BackgroundColor(Color::WHITE),
                    DespawnOnExit(AppState::Vocabulary),
    ))
    .with_children(|parent| {
        spawn_back_button(parent, AppState::Menu);

        // Section 1: About You
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|section| {
            section.spawn((
                Text::new("Run V13.py for words with mnemonic combo. Have a wonderful day :)"),
                           TextFont { font_size: 50.0, ..default() },
                           TextColor(Srgba::hex("D4AF37").unwrap().into()),
            ));

        });

    });
}
