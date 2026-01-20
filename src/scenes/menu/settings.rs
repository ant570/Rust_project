use crate::GameState;
use crate::scenes::menu::OnMenuScreen;
use bevy::color::palettes::css::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::RelativeCursorPosition;
use bevy::ui::ZIndex;

type LabelButtonsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static SettingsButtonAction,
        &'static ScoreLabel,
    ),
    (Changed<Interaction>, With<Button>),
>;

type BackButtonsQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static SettingsButtonAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(SystemParam)]
pub struct SettingsUiQuery<'w, 's> {
    pub settings: ResMut<'w, Settings>,

    pub sliders: Query<
        'w,
        's,
        (
            &'static Interaction,
            &'static RelativeCursorPosition,
            &'static VolumeSlider,
            &'static Children,
        ),
    >,

    pub handles: Query<'w, 's, &'static mut Node, With<SliderHandle>>,

    pub label_buttons: LabelButtonsQuery<'w, 's>,

    pub back_buttons: BackButtonsQuery<'w, 's>,

    pub labels: Query<'w, 's, (&'static mut Text, &'static ScoreLabel)>,

    pub next_state: ResMut<'w, NextState<crate::scenes::game_state::GameState>>,
    pub current_state: Res<'w, State<crate::scenes::game_state::GameState>>,
}

#[derive(Component)]
pub struct WinScoreLabel;

//Typy przycisków
#[derive(Component, PartialEq)]
pub enum SettingsButtonAction {
    Back,
    Plus,
    Minus,
}

//Typy wyświetlanych ustawień punktacji
#[derive(Component, Clone)]
pub enum ScoreLabel {
    Win,
    Fall,
    Coin,
    Hit,
    Damage,
}

//Ustawienia gry, które można edytować w menu ustawień
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

//Domyślne ustawienia gry
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

//Wartości dodawane lub odejmowane przy zmianie ustawień punktacji
pub const WIN_ADD: u32 = 100;
pub const FALL_DAMAGE_ADD: u32 = 25;
pub const COIN_ADD: u32 = 5;
pub const HIT_ADD: u32 = 1;
pub const DAMAGE_ADD: u32 = 10;

//Struktura suwaka głośności
#[derive(Component)]
pub struct VolumeSlider {
    pub setting_type: AudioSettingType,
}

//Typy ustawień dźwięku
#[derive(PartialEq)]
pub enum AudioSettingType {
    Music,
    Coin,
    Jump,
    Hit,
    Fail,
    Damage,
}

//Znacznik uchwytu suwaka
#[derive(Component)]
pub struct SliderHandle;

//Funkcja spawnowania pojedyńczego suwaka głośności
pub fn spawn_volume_slider(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    setting_type: AudioSettingType,
    initial_value: f32,
) {
    parent
        .spawn((Node {
            width: Val::Px(450.0),
            height: Val::Px(50.0),
            margin: UiRect::vertical(Val::Px(10.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },))
        .with_children(|slider| {
            //spawn suwaka
            slider.spawn((
                Text::new(label),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    width: Val::Px(100.0),
                    ..default()
                },
            ));

            slider
                .spawn((
                    //spawn toru suwaka
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
                    //spawn uchwytu suwaka
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

//Spawnowanie całego menu ustawień
pub fn spawn_settings(mut commands: Commands, settings: ResMut<Settings>) {
    commands
        .spawn((
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
                TextFont {
                    font_size: 70.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            //spawnowanie suwaków głośności
            spawn_volume_slider(parent, "Music Volume", AudioSettingType::Music, 0.5);
            spawn_volume_slider(parent, "Coin Volume", AudioSettingType::Coin, 0.5);
            spawn_volume_slider(parent, "Jump Volume", AudioSettingType::Jump, 0.5);
            spawn_volume_slider(parent, "Hit Volume", AudioSettingType::Hit, 0.5);
            spawn_volume_slider(parent, "Fail Volume", AudioSettingType::Fail, 0.5);
            spawn_volume_slider(parent, "Damage Volume", AudioSettingType::Damage, 0.5);

            //spawnowanie ustawień punktacji
            spawn_score_label(parent, &settings, ScoreLabel::Win);
            spawn_score_label(parent, &settings, ScoreLabel::Fall);
            spawn_score_label(parent, &settings, ScoreLabel::Coin);
            spawn_score_label(parent, &settings, ScoreLabel::Hit);
            spawn_score_label(parent, &settings, ScoreLabel::Damage);

            //Spawnowanie przyciku BACK
            parent
                .spawn((
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
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::from(GOLD)),
                    ));
                });
        });
}

//spawnowanie pojedyńczych ustawień punktacji
pub fn spawn_score_label(
    parent: &mut ChildSpawnerCommands,
    settings: &Settings,
    label_type: ScoreLabel,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|row| {
            //przycisk minus
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
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::from(WHITE)),
                ));
            });
            let text = match label_type {
                ScoreLabel::Win => "Win Score: ",
                ScoreLabel::Fall => "Fall Damage Score: ",
                ScoreLabel::Coin => "Coin Collection Score: ",
                ScoreLabel::Hit => "Hit Enemy Score: ",
                ScoreLabel::Damage => "Damage Taken Score: ",
            };
            row.spawn((
                Text::new(text),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
            ));

            row.spawn((
                Text::new(match label_type {
                    //ustawiona wartość punktacji
                    ScoreLabel::Win => settings.win_score.to_string(),
                    ScoreLabel::Fall => settings.fall_score.to_string(),
                    ScoreLabel::Coin => settings.coin_score.to_string(),
                    ScoreLabel::Hit => settings.hit_score.to_string(),
                    ScoreLabel::Damage => settings.damage_score.to_string(),
                }),
                label_type.clone(),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                WinScoreLabel,
            ));

            //przycisk plus
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
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::from(WHITE)),
                ));
            });
        });
}
pub fn settings_action(mut ui: SettingsUiQuery) {
    //obsługa suwaków
    for (interaction, cursor_pos, slider, children) in &ui.sliders {
        if *interaction == Interaction::Pressed
            && let Some(pos) = cursor_pos.normalized
        {
            let x = (pos.x + 0.5).clamp(0.0, 1.0);

            match slider.setting_type {
                AudioSettingType::Music => ui.settings.music_volume = x,
                AudioSettingType::Coin => ui.settings.coin_volume = x,
                AudioSettingType::Jump => ui.settings.jump_volume = x,
                AudioSettingType::Hit => ui.settings.hit_volume = x,
                AudioSettingType::Fail => ui.settings.fail_volume = x,
                AudioSettingType::Damage => ui.settings.damage_volume = x,
            }

            for child in children.iter() {
                if let Ok(mut handle_node) = ui.handles.get_mut(child) {
                    let x = x.clamp(0.04, 0.96);
                    handle_node.left = Val::Percent(x * 100.0 - 4.0);
                    break;
                }
            }
        }
    }

    //obsługa przycisków zmiany punktacji
    for (interaction, action, label_type) in &ui.label_buttons {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::Plus => match label_type {
                    ScoreLabel::Win => {
                        ui.settings.win_score = ui.settings.win_score.saturating_add(WIN_ADD)
                    }
                    ScoreLabel::Fall => {
                        ui.settings.fall_score =
                            ui.settings.fall_score.saturating_add(FALL_DAMAGE_ADD)
                    }
                    ScoreLabel::Coin => {
                        ui.settings.coin_score = ui.settings.coin_score.saturating_add(COIN_ADD)
                    }
                    ScoreLabel::Hit => {
                        ui.settings.hit_score = ui.settings.hit_score.saturating_add(HIT_ADD)
                    }
                    ScoreLabel::Damage => {
                        ui.settings.damage_score =
                            ui.settings.damage_score.saturating_add(DAMAGE_ADD)
                    }
                },
                SettingsButtonAction::Minus => match label_type {
                    ScoreLabel::Win => {
                        ui.settings.win_score = ui.settings.win_score.saturating_sub(WIN_ADD)
                    }
                    ScoreLabel::Fall => {
                        ui.settings.fall_score =
                            ui.settings.fall_score.saturating_sub(FALL_DAMAGE_ADD)
                    }
                    ScoreLabel::Coin => {
                        ui.settings.coin_score = ui.settings.coin_score.saturating_sub(COIN_ADD)
                    }
                    ScoreLabel::Hit => {
                        ui.settings.hit_score = ui.settings.hit_score.saturating_sub(HIT_ADD)
                    }
                    ScoreLabel::Damage => {
                        ui.settings.damage_score =
                            ui.settings.damage_score.saturating_sub(DAMAGE_ADD)
                    }
                },
                _ => {}
            }

            //aktualizacja wyświetlanych wartości punktacji
            for (mut text, label_type) in &mut ui.labels {
                text.0 = match label_type {
                    ScoreLabel::Win => ui.settings.win_score.to_string(),
                    ScoreLabel::Fall => ui.settings.fall_score.to_string(),
                    ScoreLabel::Coin => ui.settings.coin_score.to_string(),
                    ScoreLabel::Hit => ui.settings.hit_score.to_string(),
                    ScoreLabel::Damage => ui.settings.damage_score.to_string(),
                };
            }
        }
    }

    //obsługa przyciku BACK w zależności od stanu gry (z którego menu zostało wywołane)
    for (interaction, action) in &ui.back_buttons {
        if *interaction == Interaction::Pressed && matches!(action, SettingsButtonAction::Back) {
            match ui.current_state.get() {
                GameState::SettingsStart => ui.next_state.set(GameState::StartMenu),
                GameState::SettingsPause => ui.next_state.set(GameState::Paused),
                _ => {}
            }
        }
    }
}
