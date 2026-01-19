
use bevy::prelude::*;
use bevy::color::palettes::css::*;
use crate::scenes::menu::OnMenuScreen;
use bevy::ui::RelativeCursorPosition;
use crate::GameState;
use bevy::ui::FocusPolicy;
use bevy::ui::ZIndex;
#[derive(Component)]
pub struct WinScoreLabel;

#[derive(Component, PartialEq)]
pub enum SettingsButtonAction {
   Back,
   Plus,
   Minus,
}

#[derive(Component, Clone)]
pub enum LabelType{
    WinScore,
    FallScore,
    CoinScore,
    HitScore,
    DamageScore
}

#[derive(Resource)]
pub struct Settings {
    pub music_volume: f32,
    pub coin_volume: f32,
    pub jump_volume: f32,
    pub hit_volume: f32,
    pub fail_volume: f32,
    pub damage_volume: f32,
    pub win_score: u32,
    pub fall_score: u32,
    pub coin_score: u32,
    pub hit_score: u32,
    pub damage_score: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            music_volume: 0.5,
            coin_volume: 0.5,
            jump_volume: 0.5,
            hit_volume: 0.5,
            fail_volume: 0.5,
            damage_volume: 0.5,
            win_score: 1000,
            fall_score: 200,
            coin_score: 25,
            hit_score: 1,
            damage_score: 10,
        }
    }
}

pub const WIN_ADD: u32 = 100;
pub const FALL_DAMAGE_ADD: u32 = 25;
pub const COIN_ADD: u32 = 5;
pub const HIT_ADD: u32 = 1;
pub const DAMAGE_ADD: u32 = 10; 

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
            width: Val::Px(450.0),
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
                    left: Val::Percent(initial_value * 100.0 - 4.0),
                    top: Val::Px(-9.0),
                    ..default()
                },
                BackgroundColor(Color::from(GOLD)),
                ZIndex(1),
                SliderHandle,
                FocusPolicy::Pass,
            ));
        });
    });
}


pub fn spawn_settings(
    mut commands: Commands,
    settings: ResMut<Settings>,
) {
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

        spawn_volume_slider(
            parent,
            "Coin Volume",
            AudioSettingType::Coin,
            0.5,
        );

        spawn_volume_slider(
            parent,
            "Jump Volume",
            AudioSettingType::Jump,
            0.5,
        );

        spawn_volume_slider(
            parent,
            "Hit Volume",
            AudioSettingType::Hit,
            0.5,
        );

        spawn_volume_slider(
            parent,
            "Fail Volume",
            AudioSettingType::Fail,
            0.5,
        );

        spawn_volume_slider(
            parent,
            "Damage Volume",
            AudioSettingType::Damage,
            0.5,
        );

        spawn_score_label(parent, &*settings, LabelType::WinScore);
        spawn_score_label(parent, &*settings, LabelType::FallScore);
        spawn_score_label(parent, &*settings, LabelType::CoinScore);
        spawn_score_label(parent, &*settings, LabelType::HitScore);
        spawn_score_label(parent, &*settings, LabelType::DamageScore);


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

pub fn spawn_score_label(
    parent: &mut ChildSpawnerCommands,
    settings: &Settings,
    label_type: LabelType,
) {
    parent.spawn(Node {
            flex_direction: FlexDirection::Row, 
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            ..default()
        }).with_children(|row| {
            

            row.spawn((
                Button,
                SettingsButtonAction::Minus,
                label_type.clone(),
                OnMenuScreen,
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::from(RED)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("-"),
                    TextFont { font_size: 50.0, ..default() },
                    TextColor(Color::from(WHITE)),
                ));
            });
            let text = match label_type {
                LabelType::WinScore => "Win Score: ",
                LabelType::FallScore => "Fall Damage Score: ",
                LabelType::CoinScore => "Coin Collection Score: ",
                LabelType::HitScore => "Hit Enemy Score: ",
                LabelType::DamageScore => "Damage Taken Score: ",
            };
            row.spawn((
                Text::new(text),
                TextFont { font_size: 25.0, ..default() },
                TextColor(Color::from(GOLD)),
                
            ));

            row.spawn((
                Text::new(match label_type {
                    LabelType::WinScore => settings.win_score.to_string(),
                    LabelType::FallScore => settings.fall_score.to_string(),
                    LabelType::CoinScore => settings.coin_score.to_string(),
                    LabelType::HitScore => settings.hit_score.to_string(),
                    LabelType::DamageScore => settings.damage_score.to_string(),
                }),
                label_type.clone(),
                TextFont { font_size: 25.0, ..default() },
                TextColor(Color::from(GOLD)),
                WinScoreLabel,
            ));

            row.spawn((
                Button,
                SettingsButtonAction::Plus,
                OnMenuScreen,
                label_type.clone(),
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::from(GREEN)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("+"),
                    TextFont { font_size: 50.0, ..default() },
                    TextColor(Color::from(WHITE)),
                ));
            });
        });
}

pub fn settings_action(
    mut settings: ResMut<Settings>,
    interaction_query: Query<(&Interaction, &RelativeCursorPosition, &VolumeSlider, &Children)>,
    mut handle_query: Query<&mut Node, With<SliderHandle>>,
    button_label_query: Query<(&Interaction, &SettingsButtonAction, &LabelType), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    current_state: Res<State<crate::scenes::game_state::GameState>>,
    mut label_query: Query<(&mut Text, &LabelType)>,
    button_query: Query<(&Interaction, &SettingsButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, cursor_pos, slider, children) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(pos) = cursor_pos.normalized {
                let x = pos.x + 0.5;
                let clamped_x = x.clamp(0.0, 1.0);
                match slider.setting_type {
                    AudioSettingType::Music => settings.music_volume = clamped_x,
                    AudioSettingType::Coin => settings.coin_volume = clamped_x,
                    AudioSettingType::Jump => settings.jump_volume = clamped_x,
                    AudioSettingType::Hit => settings.hit_volume = clamped_x,
                    AudioSettingType::Fail => settings.fail_volume = clamped_x,
                    AudioSettingType::Damage => settings.damage_volume = clamped_x,
                }
                for child in children.iter() {
                    if let Ok(mut handle_node) = handle_query.get_mut(child) {
                        let x = clamped_x.clamp(0.04, 0.96);
                        handle_node.left = Val::Percent(x * 100.0 - 4.0);
                        break;
                    }
                }
            }
        }
    }

    for (interaction, action, label_type) in &button_label_query {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::Plus => {
                    println!("Huhu");
                    match label_type {
                        LabelType::WinScore => settings.win_score = settings.win_score.saturating_add(WIN_ADD),
                        LabelType::FallScore => settings.fall_score = settings.fall_score.saturating_add(FALL_DAMAGE_ADD),
                        LabelType::CoinScore => settings.coin_score = settings.coin_score.saturating_add(COIN_ADD),
                        LabelType::HitScore => settings.hit_score = settings.hit_score.saturating_add(HIT_ADD),
                        LabelType::DamageScore => settings.damage_score = settings.damage_score.saturating_add(DAMAGE_ADD),
                    }
                    println!("Win score increased to {}", settings.win_score);
                    for (mut text, label_type) in &mut label_query {
                        match label_type {
                            LabelType::WinScore => text.0 = settings.win_score.to_string(),
                            LabelType::FallScore => text.0 = settings.fall_score.to_string(),
                            LabelType::CoinScore => text.0 = settings.coin_score.to_string(),
                            LabelType::HitScore => text.0 = settings.hit_score.to_string(),
                            LabelType::DamageScore => text.0 = settings.damage_score.to_string(),
                        }
                    }
                }
                SettingsButtonAction::Minus => {
                    match label_type {
                        LabelType::WinScore => settings.win_score = settings.win_score.saturating_sub(WIN_ADD),
                        LabelType::FallScore => settings.fall_score = settings.fall_score.saturating_sub(FALL_DAMAGE_ADD),
                        LabelType::CoinScore => settings.coin_score = settings.coin_score.saturating_sub(COIN_ADD),
                        LabelType::HitScore => settings.hit_score = settings.hit_score.saturating_sub(HIT_ADD),
                        LabelType::DamageScore => settings.damage_score = settings.damage_score.saturating_sub(DAMAGE_ADD),
                    }
                    for (mut text, label_type) in &mut label_query {
                        match label_type {
                            LabelType::WinScore => text.0 = settings.win_score.to_string(),
                            LabelType::FallScore => text.0 = settings.fall_score.to_string(),
                            LabelType::CoinScore => text.0 = settings.coin_score.to_string(),
                            LabelType::HitScore => text.0 = settings.hit_score.to_string(),
                            LabelType::DamageScore => text.0 = settings.damage_score.to_string(),
                        }
                    }
                }
                
                _ => {}
            }
        }
    }

    for (interaction, action) in &button_query {
        if *interaction == Interaction::Pressed {
            if let SettingsButtonAction::Back = action {
                match current_state.get() {
                    GameState::SettingsStart => {
                        next_state.set(GameState::StartMenu);
                    }
                    GameState::SettingsPause => {
                        next_state.set(GameState::Paused);
                    }
                    _ => {}
                }
            }
        }
    }      
}
