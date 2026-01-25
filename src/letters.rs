use bevy::prelude::*;
use crate::{AppState, spawn_back_button};
use serde::{Deserialize, Serialize};
use std::fs;

// --- Resources ---

#[derive(Resource, Default)]
pub struct SelectedLetterResource {
    pub capital: String,
    pub small: String,
    pub polish_word: String,
    pub sound: String,
    pub index: usize, // We need to track the index to know what is "Next"
}

// We store the full list of letters here so we can access them in the Detail screen
#[derive(Resource, Default)]
pub struct LettersData {
    pub letters: Vec<Letter>,
}
// --- Components ---

#[derive(Component)]
pub struct LetterButton {
    pub capital: String,
    pub small: String,
    pub polish_word: String,
    pub sound: String,
    pub index: usize,
}

// Marker components so we can update the text when "Next" is clicked
#[derive(Component)]
pub struct CapitalDisplay;

#[derive(Component)]
pub struct AudioButton;

#[derive(Component)]
pub struct WordDisplay;

#[derive(Component)]
pub struct SmallDisplay;

#[derive(Component)]
pub struct NextButton;

#[derive(Deserialize, Serialize, Debug, Clone)] // Added Clone
pub struct Letter {
    pub polish_letter_small: String,
    pub polish_letter_capital: String,
    pub polish_word: String,
    pub sound: String,
}

// --- Systems ---

pub fn spawn_letters_screen(mut commands: Commands, mut letters_data: ResMut<LettersData>) {
    // 1. Load data only if we haven't already (or reload to be safe)
    if letters_data.letters.is_empty() {
        let path = "assets/letters/letters.ron";
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        letters_data.letters = ron::from_str(&contents).expect("Failed to parse RON");
    }

    // 2. Main Screen Container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        DespawnOnExit(AppState::Letters),
    ))
    .with_children(|parent| {
        spawn_back_button(parent, AppState::Menu);
        parent.spawn((
            Text::new("Letters"),
            TextFont { font_size: 60.0, ..default() },
            TextColor(Srgba::hex("DC143C").unwrap().into()),
            Node {
       //         margin: UiRect::bottom(Val::Px(30.0)), // Space between title and grid
                ..default()
            },
        ));
        // 3. Grid Container for Letters
        parent.spawn(Node {
            display: Display::Flex,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            max_width: Val::Px(1200.0),
            column_gap: Val::Px(10.0),
            row_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|grid| {
            // Iterate with index using .enumerate()
            for (i, letter_entry) in letters_data.letters.iter().enumerate() {
                grid.spawn((
                    Button,
                    LetterButton {
                        capital: letter_entry.polish_letter_capital.clone(),
                        small: letter_entry.polish_letter_small.clone(),
                        polish_word: letter_entry.polish_word.clone(),
                        sound: letter_entry.sound.clone(),
                        index: i,
                    },
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(&letter_entry.polish_letter_capital),
                        TextColor(Srgba::hex("DC143C").unwrap().into()),
                        TextFont { 
                            font_size: 32.0, 
                            ..default() 
                        },
                    ));
                });
            }
        });
    });
}

// Handles clicking a letter from the Grid
pub fn handle_letter_click(
    mut next_state: ResMut<NextState<AppState>>,
    mut selected_resource: ResMut<SelectedLetterResource>,
    interaction_query: Query<(&Interaction, &LetterButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, letter_btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            selected_resource.capital = letter_btn.capital.clone();
            selected_resource.small = letter_btn.small.clone();
            selected_resource.polish_word=letter_btn.polish_word.clone();
            selected_resource.sound=letter_btn.sound.clone();
            selected_resource.index = letter_btn.index;
            
            next_state.set(AppState::LetterDetail);
        }
    }
}

pub fn spawn_letter_detail_screen(
    mut commands: Commands,
    selected: Res<SelectedLetterResource>
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
        },
        BackgroundColor(Color::WHITE),
                    DespawnOnExit(AppState::LetterDetail),
    ))
    .with_children(|parent| {
        spawn_back_button(parent, AppState::Letters);

        // --- NEXT BUTTON (Bottom Right) ---
        parent.spawn((
            Button,
            NextButton,
            Node {
                width: Val::Px(100.0),
                      height: Val::Px(40.0),
                      position_type: PositionType::Absolute,
                      bottom: Val::Px(20.0),
                      right: Val::Px(20.0),
                      justify_content: JustifyContent::Center,
                      align_items: AlignItems::Center,
                      ..default()
            },
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("Next"),
                       TextFont { font_size: 18.0, ..default() },
                       TextColor(Srgba::hex("DC143C").unwrap().into()),
            ));
        });

        // --- LETTERS DISPLAY ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|container| {
            container.spawn((
                Text::new(&selected.capital),
                             TextFont { font_size: 150.0, ..default() },
                             TextColor(Srgba::hex("DC143C").unwrap().into()),
                             CapitalDisplay,
            ));

            container.spawn((
                Text::new(&selected.small),
                             TextFont { font_size: 150.0, ..default() },
                             TextColor(Srgba::hex("DC143C").unwrap().into()),
                             SmallDisplay,
            ));

            container.spawn((
                Text::new(&selected.polish_word),
                             TextFont { font_size: 80.0, ..default() },
                             TextColor(Srgba::hex("DC143C").unwrap().into()),
                             Node {
                                 margin: UiRect::top(Val::Px(20.0)),
                             ..default()
                             },
                             WordDisplay,
            ));

            container.spawn((
                Button,
                AudioButton,
                Node {
                    width: Val::Px(160.0),
                             height: Val::Px(50.0),
                             justify_content: JustifyContent::Center,
                             align_items: AlignItems::Center,
                             margin: UiRect::top(Val::Px(10.0)),
                             ..default()
                },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("Listen"),
                           TextFont { font_size: 22.0, ..default() },
                           TextColor(Srgba::hex("DC143C").unwrap().into()),
                ));
            });
        });
    });
}

// Logic for clicking the "Next" button inside the Detail screen
pub fn handle_next_click(
    mut selected: ResMut<SelectedLetterResource>,
    letters_data: Res<LettersData>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
    mut capital_query: Query<&mut Text, (With<CapitalDisplay>, Without<SmallDisplay>, Without<WordDisplay>)>,
    mut small_query: Query<&mut Text, (With<SmallDisplay>, Without<CapitalDisplay>, Without<WordDisplay>)>,
    mut polish_word_query: Query<&mut Text, (Without<SmallDisplay>, Without<CapitalDisplay>, With<WordDisplay>)>,
    mut sound_query: Query<&mut Text, (Without<SmallDisplay>, Without<CapitalDisplay>, Without<WordDisplay>)>,
) {
    if let Ok(interaction) = interaction_query.single() {
        if *interaction == Interaction::Pressed {
            // 1. Calculate next index (wrap around using modulo)
            let next_index = (selected.index + 1) % letters_data.letters.len();
            
            // 2. Get the new letter data
            let next_letter = &letters_data.letters[next_index];
            
            // 3. Update Resource
            selected.index = next_index;
            selected.capital = next_letter.polish_letter_capital.clone();
            selected.small = next_letter.polish_letter_small.clone();
            selected.polish_word = next_letter.polish_word.clone();
            selected.sound=next_letter.sound.clone();

            // 4. Update the Text on screen immediately
            if let Ok(mut txt) = capital_query.single_mut() {
                txt.0 = selected.capital.clone();
            }
            if let Ok(mut txt) = small_query.single_mut() {
                txt.0 = selected.small.clone();
            }
            if let Ok(mut txt) = polish_word_query.single_mut(){
                txt.0=selected.polish_word.clone();
            }
            if let Ok(mut txt) = sound_query.single_mut(){
                txt.0=selected.sound.clone();
            }
        }
    }
}
pub fn handle_audio_click(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    selected: Res<SelectedLetterResource>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<AudioButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // The path usually looks like "letters/sounds/a.ogg" 
            // based on your RON file data
            let sound_path = format!("letters/{}", selected.sound);
            
           commands.spawn(AudioPlayer::new(
        asset_server.load(sound_path),
        //PlaybackSettings::ONCE,
    ));//.with_settings(PlaybackSettings::ONCE)); 
        }
    }
}
