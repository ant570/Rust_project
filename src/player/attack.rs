use crate::player::player::{Control, Player};
use crate::player::spawn::ChargeBar;
use crate::player::spawn::Collider;
use crate::player::spawn::ScoreText;
use crate::world::utils::WORLD_WIDTH;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::Time;

pub const STICK_BASE_RANGE: f32 = 180.0;
pub const STICK_THICKNESS: f32 = 20.0;
const SWING_DURATION: f32 = 0.15;
const CHARGE_RATE: f32 = 1.0;
const MAX_CHARGE: f32 = 1.5;
const FORCE_BASE: f32 = 50.0; // When only pressed
const FORCE_PER_CHARGE: f32 = 700.0; // When fully charged

#[derive(Component)]
pub struct AttackState {
    pub charge: f32,
    pub facing: f32,
    pub was_pressed: bool,
    pub just_released: bool,
    pub release_charge: f32,
    pub swing_timer: f32,
}

#[derive(Component)]
pub struct Stick;

fn compute_reach() -> f32 {
    STICK_BASE_RANGE.min(WORLD_WIDTH / 2.0)
}

fn rects_intersect(min_a: Vec2, max_a: Vec2, min_b: Vec2, max_b: Vec2) -> bool {
    min_a.x < max_b.x && max_a.x > min_b.x && min_a.y < max_b.y && max_a.y > min_b.y
}
pub fn stick_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<
        (
            Entity,
            &mut Transform,
            &Collider,
            &mut Player,
            &mut AttackState,
            &Children,
        ),
        Without<Stick>,
    >,
    mut sticks: Query<(&mut Transform, &mut Sprite), (With<Stick>, Without<Player>)>,
    mut score_texts: Query<&mut Transform, (With<ScoreText>, Without<Player>, Without<Stick>)>,
    mut charge_bars: Query<
        (&mut Transform, &mut Sprite),
        (
            With<ChargeBar>,
            Without<Player>,
            Without<Stick>,
            Without<ScoreText>,
        ),
    >,
) {
    for (_entity, mut transform, collider, player, mut state, children) in players.iter_mut() {
        let (left_pressed, right_pressed, attack_pressed) = match player.control {
            Control::Wasd => (
                keyboard_input.pressed(KeyCode::KeyA),
                keyboard_input.pressed(KeyCode::KeyD),
                keyboard_input.pressed(KeyCode::KeyF),
            ),
            Control::Arrows => (
                keyboard_input.pressed(KeyCode::ArrowLeft),
                keyboard_input.pressed(KeyCode::ArrowRight),
                keyboard_input.pressed(KeyCode::Period),
            ),
        };

        if left_pressed && !right_pressed {
            state.facing = -1.0;
        } else if right_pressed && !left_pressed {
            state.facing = 1.0;
        }

        if state.facing.abs() < f32::EPSILON {
            state.facing = 1.0;
        }

        transform.scale.x = state.facing;

        if attack_pressed {
            state.charge = (state.charge + time.delta_secs() * CHARGE_RATE).min(MAX_CHARGE);
            state.just_released = false;
        } else if state.was_pressed {
            state.just_released = true;
            state.release_charge = state.charge;
            state.charge = 0.0;
            state.swing_timer = SWING_DURATION;
        } else {
            state.just_released = false;
            state.charge = 0.0;
        }
        state.was_pressed = attack_pressed;
        if state.swing_timer > 0.0 {
            state.swing_timer = (state.swing_timer - time.delta_secs()).max(0.0);
        }

        let reach = compute_reach();
        for child in children.iter() {
            if let Ok((mut stick_transform, mut stick_sprite)) = sticks.get_mut(child) {
                if state.swing_timer > 0.0 {
                    // Attack: Horizontal
                    stick_sprite.custom_size = Some(Vec2::new(reach, STICK_THICKNESS));
                    stick_transform.translation.x = reach / 2.0;
                    stick_transform.translation.y = 0.0;
                    stick_transform.rotation = Quat::IDENTITY;
                } else {
                    // Default: Vertical
                    stick_sprite.custom_size = Some(Vec2::new(STICK_THICKNESS, reach));
                    stick_transform.translation.x = collider.half_size.x + STICK_THICKNESS * 1.0;
                    stick_transform.translation.y = 0.0;
                    stick_transform.rotation = Quat::IDENTITY;
                }
            }

            if let Ok(mut text_transform) = score_texts.get_mut(child) {
                text_transform.scale.x = state.facing; // Counter-flip text so it remains readable
            }

            if let Ok((mut _bar_transform, mut bar_sprite)) = charge_bars.get_mut(child) {
                if state.charge > 0.0 {
                    let charge_ratio = state.charge / MAX_CHARGE;
                    let bar_width = 100.0 * charge_ratio;
                    bar_sprite.custom_size = Some(Vec2::new(bar_width, 10.0));
                    bar_sprite.color = Color::srgb(charge_ratio, 1.0 - charge_ratio, 0.0);
                } else {
                    bar_sprite.custom_size = Some(Vec2::new(0.0, 10.0));
                }
            }
        }
    }

    let mut combinations = players.iter_combinations_mut();
    while let Some([player1, player2]) = combinations.fetch_next() {
        let (_entity1, mut transform1, collider1, mut player1_component, state1, _children1) =
            player1;
        let (_entity2, mut transform2, collider2, mut player2_component, state2, _children2) =
            player2;

        if state1.just_released {
            let reach = compute_reach();
            let center =
                transform1.translation.truncate() + Vec2::new(state1.facing * reach / 2.0, 0.0);
            let half = Vec2::new(reach / 2.0, STICK_THICKNESS / 2.0);
            let stick_min = center - half;
            let stick_max = center + half;

            let target_min = transform2.translation.truncate() - collider2.half_size;
            let target_max = transform2.translation.truncate() + collider2.half_size;

            if rects_intersect(stick_min, stick_max, target_min, target_max) {
                let push = FORCE_BASE + state1.release_charge * FORCE_PER_CHARGE;
                transform2.translation.x += state1.facing * push;
                player2_component.pos.x = transform2.translation.x;
            }
        }

        if state2.just_released {
            let reach = compute_reach();
            let center =
                transform2.translation.truncate() + Vec2::new(state2.facing * reach / 2.0, 0.0);
            let half = Vec2::new(reach / 2.0, STICK_THICKNESS / 2.0);
            let stick_min = center - half;
            let stick_max = center + half;

            let target_min = transform1.translation.truncate() - collider1.half_size;
            let target_max = transform1.translation.truncate() + collider1.half_size;

            if rects_intersect(stick_min, stick_max, target_min, target_max) {
                let push = FORCE_BASE + state2.release_charge * FORCE_PER_CHARGE;
                transform1.translation.x += state2.facing * push;
                player1_component.pos.x = transform1.translation.x;
            }
        }
    }
}
