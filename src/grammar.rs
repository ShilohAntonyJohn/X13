// src/grammar.rs
use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use crate::{AppState, spawn_back_button};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use rand::Rng;
use bevy::window::PrimaryWindow;
#[derive(Component)]
pub struct PlayerWord;

#[derive(Component)]
pub struct FallingSuffix {
    pub speed: f32,
}

const POLISH_SUFFIXES: &[&str] = &[
    "-y", "-o",  "-a", "-e", "-ę", "-ie", "-u", "-i", "-ą", 
    "-owi", "-ami", "-em", "-ym", "-im",  "-ego", "-emu", "-ej"
];
#[derive(Resource, Default)]
pub struct SelectedCase {
   pub name: String,
   pub details: Option<CaseInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CaseInfo {
    pub description: String,
    pub genders: HashMap<String, GenderRules>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenderRules {
    pub noun_suffix: String,
    pub adj_suffix: String,
    pub pronoun_example:Option<String>, // Changed to match your design choice
}
pub fn spawn_grammar_screen(mut commands: Commands) {
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
        DespawnOnExit(AppState::Grammar),
    ))
    .with_children(|parent| {
        spawn_back_button(parent, AppState::Menu);

        parent.spawn((
            Text::new("The 7 Polish Cases"),
            TextFont { font_size: 50.0, ..default() },
            TextColor(Srgba::hex("DC143C").unwrap().into()), 
            Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }
        ));

        // List of cases: (Polish Name, English Name)
        let cases = [
            ("Mianownik", "Nominative"),
            ("Dopełniacz", "Genitive"),
            ("Celownik", "Dative"),
            ("Biernik", "Accusative"),
            ("Narzędnik", "Instrumental"),
            ("Miejscownik", "Locative"),
            ("Wołacz", "Vocative"),
//	    ("Liczby", "Numericals"),
        ];

        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        }).with_children(|list| {
            for (pl, en) in cases {
                let case_string = format!("{} ({})", pl, en);
		let case_string = case_string.clone();
		list.spawn((
                    Button,
                    Node {
                        width: Val::Px(350.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(format!("{} ({})", pl, en)),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Srgba::hex("DC143C").unwrap().into()),
                    ));
                })
.observe(move |_trigger: On<Pointer<Click>>, 
                           mut next_state: ResMut<NextState<AppState>>,
                           mut selected: ResMut<SelectedCase>| {
                // 1. Save the case name to the Resource
		let path = "assets/grammar/cases.ron";
        let contents = fs::read_to_string(path).expect("Failed to read cases.ron");
        
        // 2. Parse the entire map of cases
        let all_cases: HashMap<String, CaseInfo> = ron::from_str(&contents)
            .expect("Failed to parse grammar RON");
                selected.name = case_string.clone();
		selected.details = all_cases.get(&case_string).cloned();
                // 2. Transition to the blank screen
                next_state.set(AppState::CaseDetail);                
});
            }
        });
    });
}
pub fn p13(mut commands: Commands, selected: Res<SelectedCase>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::WHITE),
        DespawnOnExit(AppState::CaseDetail),
    )).with_children(|parent| {
        spawn_back_button(parent, AppState::Grammar);

        // Display the name of the case that was clicked!
        parent.spawn((
            Text::new(format!(" {}", selected.name)),
            TextFont { font_size: 40.0, ..default() },
            TextColor(Srgba::hex("DC143C").unwrap().into()),
        ));
	parent.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart, // Everything inside here goes to the LEFT
            row_gap: Val::Px(20.0),
            ..default()
        }).with_children(|left_side| {
            
            if let Some(details) = &selected.details {
                // DESCRIPTION
                left_side.spawn((
                    Text::new(&details.description),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Srgba::hex("DC143C").unwrap().into()),
                    Node {
                        max_width: Val::Px(800.0), // Prevents text from stretching too far
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    }
                ));

                // GENDER RULES LIST
                for (gender, rules) in &details.genders {
                    left_side.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    }).with_children(|item| {
                        // Gender Label
                        item.spawn((
                            Text::new(gender),
                            TextFont { font_size: 26.0, ..default() },
                            TextColor(Srgba::hex("DC143C").unwrap().into()),
                        ));

                        // Suffixes
                        item.spawn((
                            Text::new(format!("• Noun: {}", rules.noun_suffix)),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Srgba::hex("DC143C").unwrap().into()),
                        ));
                        item.spawn((
                            Text::new(format!("• Adjective: {}", rules.adj_suffix)),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Srgba::hex("DC143C").unwrap().into()),
                        ));

                        // Pronoun Example (using the field name from your struct)
                        item.spawn((
//                            Text::new(format!("• Example: {}", rules.pronoun)),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Srgba::hex("DC143C").unwrap().into()),
                        ));
                    });
                }
            }
        });
    });
	//RHS
	let target_label = get_random_target(&selected); 
    // 2. Spawn the Player Word with the selected label
    commands.spawn((
        Text::new(target_label),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Srgba::hex("DC143C").unwrap().into()), 
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            left: Val::Percent(10.0), // Initial center position
            ..default()
        },
	ZIndex(10),	
        PlayerWord, // The marker component for the move_player system
        DespawnOnExit(AppState::CaseDetail),
    ));
	spawn_suffix_wave(&mut commands, &selected);
}

fn spawn_suffix_wave(commands: &mut Commands, selected: &SelectedCase) {
    let mut rng = rand::rng();
    let mut pool: Vec<String> = Vec::new();

    if let Some(details) = &selected.details {
        for rules in details.genders.values() {
            for s in rules.noun_suffix.split(',') {
                let trimmed = s.trim().to_string();
                if !trimmed.is_empty() { pool.push(trimmed); }
            }
            for s in rules.adj_suffix.split(',') {
                let trimmed = s.trim().to_string();
                if !trimmed.is_empty() { pool.push(trimmed); }
            }
        }
    }

    // Add 3 random "distractor" suffixes
    for _ in 0..3 {
        let random_idx = rng.random_range(0..POLISH_SUFFIXES.len());
        pool.push(POLISH_SUFFIXES[random_idx].to_string());
    }

    for suffix in pool {
        let x = rng.random_range(600.0..1400.0); // Adjusted for better screen coverage
        let y = rng.random_range(-500.0..-50.0); // Start off-screen
        let speed = rng.random_range(200.0..250.0);
        
        commands.spawn((
            Text::new(suffix),
            TextFont { font_size: 35.0, ..default() },
            TextColor(Srgba::hex("DC143C").unwrap().into()),
            Transform::from_xyz(x, y, 100.0),
            FallingSuffix { speed },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(y),
                ..default()
            },
            ZIndex(1),
            DespawnOnExit(AppState::CaseDetail),
        ));
    }
}
pub fn animate_falling_suffixes(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Node, &FallingSuffix)>,
) {
let Ok(window) = windows.single() else { return };
let window_height = window.height();
let dt = time.delta_secs();
    for (mut node, suffix) in query.iter_mut() {
// In UI space, 'top' is usually the property we manipulate
        if let Val::Px(current_top) = node.top {
            let mut new_top = current_top + (suffix.speed * dt);
            
            if new_top > window_height { 
                new_top = -50.0; 
            }
            
            node.top = Val::Px(new_top);
        }
    }
}
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Node, With<PlayerWord>>,
) {
    if let Ok(mut node) = query.single_mut() {
        if let Val::Percent(mut x) = node.left {
            let speed = 100.0; // Percent per second
            if keyboard_input.pressed(KeyCode::KeyA) {
                x -= speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                x += speed * time.delta_secs();
            }
            // Clamp within screen
            node.left = Val::Percent(x.clamp(10.0, 90.0));
        }
    }
}
fn get_random_target(selected: &SelectedCase) -> String {
    let mut rng = rand::rng();
    let word_types = ["Noun", "Adjective"];
    let selected_type = word_types[rng.random_range(0..word_types.len())];
	let gender_label = if selected.name.contains("Mianownik") || selected.name.contains("Celownik") || selected.name.contains("Narzędnik") || selected.name.contains("Miejscownik") || selected.name.contains("Wołacz") {
        // Simple genders for Nominative
        let choices = ["Masculine (All)", "Feminine", "Neuter"];
        choices[rng.random_range(0..choices.len())].to_string()
    } else {
        // Expanded genders for other cases
        let choices = [
            "Masculine (Personal/Animate)",
            "Masculine (Inanimate)",
            "Feminine",
            "Neuter"
        ];
        choices[rng.random_range(0..choices.len())].to_string()
    };
    format!("{} ({})", selected_type, gender_label)
}

pub fn check_collisions(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    selected: Res<SelectedCase>,
    mut player_query: Query<(&Node, &mut Text), With<PlayerWord>>,
    suffix_query: Query<(Entity, &Node, &Text), (With<FallingSuffix>, Without<PlayerWord>)>,
	all_suffixes_query: Query<Entity, With<FallingSuffix>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((p_node, mut p_text)) = player_query.single_mut() else { return };

    let window_width = window.width();
    let window_height = window.height();

    // 1. Calculate Player Center and Width
    // Approximate width: font_size * 0.5 per character is a safe bet for monospace/standard fonts
    let p_char_count = p_text.0.len() as f32;
    let p_width = p_char_count * 10.0; // 20.0 font_size * 0.5
    
    let p_x_left = if let Val::Percent(perc) = p_node.left {
        (perc / 100.0) * window_width
    } else {
        0.0
    };
    let p_x_center = p_x_left + (p_width / 2.0);

    // Player vertical bounds (player is at bottom: 40px)
    let p_y_top = window_height - 100.0; 
    let p_y_bottom = window_height - 20.0;

    for (s_entity, s_node, s_text) in suffix_query.iter() {
        if let (Val::Px(sx), Val::Px(sy)) = (s_node.left, s_node.top) {
            
            // 2. Calculate Suffix Width
            let s_char_count = s_text.0.len() as f32;
            let s_width = s_char_count * 17.5; // 35.0 font_size * 0.5
            let s_x_center = sx + (s_width / 2.0);

            // 3. Exact AABB (Axis-Aligned Bounding Box) Collision
            // Check if the distance between centers is less than the sum of half-widths
            let combined_half_widths = (p_width / 2.0) + (s_width / 2.0);
            let x_hit = (p_x_center - s_x_center).abs() < combined_half_widths;
            
            // Vertical hit check
            let y_hit = sy > p_y_top && sy < p_y_bottom;

            if x_hit && y_hit {
                let caught_suffix = s_text.0.as_str();
                let player_goal = p_text.0.as_str();
                let mut is_correct = false;

                if let Some(details) = &selected.details {
                    for (gender_key, rules) in &details.genders {
                        if player_goal.contains(gender_key) {
                            let suffix_string = if player_goal.contains("Noun") {
                                &rules.noun_suffix
                            } else {
                                &rules.adj_suffix
                            };

                            if suffix_string.split(',').any(|s| s.trim() == caught_suffix) {
                                is_correct = true;
                                break;
                            }
                        }
                    }
                }

                if is_correct {
                    for entity in all_suffixes_query.iter() {
                        commands.entity(entity).despawn();
                    }
			p_text.0 = get_random_target(&selected);
                    info!("Correct Catch: {}", caught_suffix);
			spawn_suffix_wave(&mut commands, &selected);
                } else {
                    // Feedback for wrong suffix
                    commands.entity(s_entity).despawn();
                    info!("Wrong Suffix: {}", caught_suffix);
                }
            }
        }
    }
}
