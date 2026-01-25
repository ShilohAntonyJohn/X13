use bevy::prelude::*;
use crate::{AppState, spawn_back_button};

pub fn spawn_about_screen(mut commands: Commands) {
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
        DespawnOnExit(AppState::About), 
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
                Text::new("About You"),
                TextFont { font_size: 50.0, ..default() },
                TextColor(Srgba::hex("D4AF37").unwrap().into()),
            ));
            section.spawn((
                Text::new("You are the most amazing person on the planet!!! :)"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Srgba::hex("DC143C").unwrap().into()), // Subtle gray subtext
            ));
        });

        // Section 2: About (The Project/Creator)
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|section| {
            section.spawn((
                Text::new("About"),
                TextFont { font_size: 50.0, ..default() },
                TextColor(Srgba::hex("D4AF37").unwrap().into()),
            ));
            section.spawn((
                Text::new("X13 Polish language learning app created by Shiloh Antony John"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Srgba::hex("DC143C").unwrap().into()),
            ));
        });
    });
}
