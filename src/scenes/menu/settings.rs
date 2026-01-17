use bevy::ecs::spawn;
use bevy::prelude::*;
use bevy::color::palettes::css::*;
use crate::scenes::menu::OnMenuScreen;
use crate::GameState;

#[derive(Component, PartialEq)]
pub enum SettingsButtonAction {
   Back
}

#[derive(Resource)]
pub struct AudioSetings {
    pub music_volume: f32,
    pub coin_volume: f32,
    pub jump_volume: f32,
    pub hit_volume: f32,
    pub fail_volume: f32,
    pub damage_volume: f32,
}

impl Default for AudioSetings {
    fn default() -> Self {
        AudioSetings {
            music_volume: 0.5,
            coin_volume: 0.5,
            jump_volume: 0.5,
            hit_volume: 0.5,
            fail_volume: 0.5,
            damage_volume: 0.5,
        }
    }
}

#[derive(Component)]
pub struct VolumeSlider{
    pub setting_type: AudioSettingType,
}

#[derive(PartialEq)]
pub enum AudioSettingType {
    Music,
    Coin,
    Jump,
    Hit,
    Fail,
    Damage,
}

#[derive(Component)]
pub struct SliderHandle;

pub fn spawn_volume_slider(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    setting_type: AudioSettingType,
    initial_value: f32,
) {
    parent.spawn((
        Node {
            width: Val::Px(400.0),
            height: Val::Px(50.0),
            margin: UiRect::vertical(Val::Px(10.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
    ))
    .with_children(|slider| {
        slider.spawn((
            Text::new(label),
            TextFont { font_size: 25.0, ..default() },
            TextColor(Color::from(GOLD)),
            Node { width: Val::Px(100.0), ..default() },
        ));

        // Slider Track
        slider.spawn((
            Node {
                width: Val::Px(250.0),
                height: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            VolumeSlider { setting_type },
            Interaction::default(),
            bevy::ui::RelativeCursorPosition::default(),

        ))
        .with_children(|track| {
            track.spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(initial_value * 100.0 - 2.5),
                    top: Val::Px(-10.0),
                    ..default()
                },
                SliderHandle,
            ));
        });
    });
}


pub fn spawn_settings(mut commands: Commands) {
    commands.spawn((
        OnMenuScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::from(BLACK)),
    ))
    
    .with_children(|parent| {
        parent.spawn((
            Text::new("Settings"),
            TextFont { font_size: 70.0, ..default() },
            TextColor(Color::from(GOLD)),
            Node { margin: UiRect::bottom(Val::Px(40.0)), ..default() },
        ));

        spawn_volume_slider(
            parent,
            "Music Volume",
            AudioSettingType::Music,
            0.5,
        );

        parent.spawn((
            Button,
            SettingsButtonAction::Back,
            OnMenuScreen,
            Node {
                width: Val::Px(500.0),
                height: Val::Px(80.0),
                margin: UiRect::top(Val::Px(40.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(Color::from(GOLD)),
            BackgroundColor(Color::from(BLACK)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("BACK"),
                TextFont { font_size: 25.0, ..default() },
                TextColor(Color::from(GOLD)),
            ));
        });
    });
    
}

pub fn settings_action(
    interaction_query: Query<
        (&Interaction, &SettingsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    current_state: Res<State<crate::scenes::game_state::GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed && *action == SettingsButtonAction::Back {
            match current_state.get() { 
                GameState::SettingsStart => {
                    if *action == SettingsButtonAction::Back {
                        next_state.set(GameState::StartMenu);
                    }
                }
                GameState::SettingsPause => {
                    if *action == SettingsButtonAction::Back {
                        next_state.set(GameState::Paused);
                    }
                }
                _ => {}
            }
        }
    }
}
