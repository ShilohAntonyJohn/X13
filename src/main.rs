use bevy::prelude::*;
mod letters;
mod vocabulary;
mod grammar;
mod about;
#[derive(Component, Clone, Copy, Debug)]
enum PolishMenuAction {
    Grammar,
    Vocabulary,
    Letters,
    About,
}
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Letters,
    Grammar,
    CaseDetail,
    LetterDetail,
    Vocabulary,
    About,
}
fn main() {
    App::new()
        // DefaultPlugins opens the window and sets up the renderer (Metal on Mac)
        .add_plugins(DefaultPlugins)
	.init_state::<AppState>()
	.init_resource::<grammar::SelectedCase>()
        .init_resource::<letters::SelectedLetterResource>()
        .init_resource::<letters::LettersData>()
        // Explicitly set the background to pure black
        .insert_resource(ClearColor(Color::WHITE))
        
        // You MUST spawn a camera to see anything, 
        // otherwise you'll just see the OS default (usually white or empty)
        .add_systems(Startup, setup_camera)
	.add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(OnEnter(AppState::Grammar), grammar::spawn_grammar_screen)
	.add_systems(OnEnter(AppState::CaseDetail), grammar::p13)
        .add_systems(
    Update, 
    (
	grammar::animate_falling_suffixes,
        grammar::move_player,
	grammar::check_collisions,
    ).run_if(in_state(AppState::CaseDetail))
)
        .add_systems(OnEnter(AppState::Letters), letters::spawn_letters_screen)
        .add_systems(Update, letters::handle_audio_click.run_if(in_state(AppState::LetterDetail)))
        .add_systems(OnEnter(AppState::Vocabulary), vocabulary::spawn_vocabulary_screen)
	.add_systems(OnEnter(AppState::About), about::spawn_about_screen)
        .add_systems(Update, letters::handle_letter_click.run_if(in_state(AppState::Letters)))
        .add_systems(OnEnter(AppState::LetterDetail), letters::spawn_letter_detail_screen)
        .add_systems(Update, letters::handle_next_click.run_if(in_state(AppState::LetterDetail)))
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer> ) {
    // 1. Root Master Node
	 let font_handle: Handle<Font> = asset_server.load("font/GoogleSansCode-Regular.ttf");
    commands.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
	DespawnOnExit(AppState::Menu),
	))
	.with_children(|parent| {
        // 2. THE ANCHOR BOX (This holds everything together)
        // We don't give this a height, so it shrinks to fit its contents
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
		margin: UiRect::top(Val::Px(150.0)),
            ..default()
        })
    .with_children(|anchor| {
	anchor.spawn((
    Button,
    PolishMenuAction::About, // Ensure you added About to your enum
    Node {
        position_type: PositionType::Absolute,
        // Match the Y-level of the Letters button:
        // The column has 40px top margin + 2 gaps (15px each) + 2 buttons (55px each)
        top: Val::Px(360.0), 
        // Move it left: Letters has -120px offset + its 180px width
        // We place this at -300px to put it to the left of Letters
        left: Val::Px(-200.0), 
        width: Val::Px(180.0),
        height: Val::Px(55.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(Color::WHITE),
))
.with_children(|btn| {
    btn.spawn((
        Text::new("About"),
        //TextFont { font_size: 22.0, ..default() },
	TextFont {
        font: font_handle.clone(),
        font_size: 22.0,
        ..default()
    },
        TextColor(Srgba::hex("DC143C").unwrap().into()),
    ));
})
.observe(|_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
    next_state.set(AppState::About);
});
        // 2. The X13 Title
	anchor.spawn((
                Text::new("X13"),
                TextFont { font_size: 150.0, ..default() },
                TextColor(Srgba::hex("DC143C").unwrap().into()),
            ));

            // 4. The Staircase Container
            // By putting it here, the gap is ALWAYS 40px, fullscreen or not.
            anchor.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                margin: UiRect::top(Val::Px(40.0)), // Fixed distance!
                ..default()
            })
            .with_children(|column| {
                let actions = [
                    (PolishMenuAction::Grammar, "Grammar", 0.0),
                    (PolishMenuAction::Vocabulary, "Vocabulary", -60.0),
                    (PolishMenuAction::Letters, "Letters", -120.0),
                ];

                for (action, label, offset) in actions {
                    column.spawn((
                        Button,
                        action,
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(55.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::left(Val::Px(offset)),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(label),
                            TextFont {
        font: font_handle.clone(),
        font_size: 22.0,
        ..default()
    },
				TextColor(Srgba::hex("DC143C").unwrap().into()),
                        ));
                    })
                    .observe(move |trigger: On<Pointer<Click>>, query: Query<&PolishMenuAction>,mut next_state: ResMut<NextState<AppState>>,| {
                        let clicked_action = query.get(trigger.event_target()).unwrap();
			 match clicked_action {
                            PolishMenuAction::Grammar=> next_state.set(AppState::Grammar),
                            PolishMenuAction::Vocabulary=> next_state.set(AppState::Vocabulary),
                            PolishMenuAction::Letters => next_state.set(AppState::Letters),
				PolishMenuAction::About => next_state.set(AppState::About),
                        }
                    });
                }
            });
        });
    });
}
pub fn spawn_back_button(parent: &mut ChildSpawnerCommands<'_>, target_state: AppState) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(100.0),
            height: Val::Px(40.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::WHITE),
    ))
    .with_children(|btn: &mut ChildSpawnerCommands<'_>| {
        btn.spawn((
            Text::new("Back"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Srgba::hex("DC143C").unwrap().into()),
        ));
    })
    .observe(move |_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
	next_state.set(target_state.clone());    
});
}
