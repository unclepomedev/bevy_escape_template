use super::{ZoomScreenElement, close_zoom_screen};
use crate::domain::input_buffer::InputBuffer;
use crate::domain::solved::{QuizAnswer, SetQuiz1Solution, WrongAnswerMessage, check_quiz1_answer};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_escape_core::{Effect, apply_effects};

#[derive(Component)]
pub struct InputFieldDisplay;

#[derive(Component)]
pub struct FeedbackDisplay;

pub fn spawn_zoom2_screen(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single().expect("primary window should exist");

    spawn_background(&mut commands, window);
    spawn_input_field(&mut commands);
    spawn_confirm_button(&mut commands);
    spawn_feedback_display(&mut commands);
}

fn spawn_background(commands: &mut Commands, window: &Window) {
    let full_screen_size = Vec2::new(window.width(), window.height());
    let overlay_background_color = Color::srgba(0.05, 0.05, 0.05, 0.95);
    let background_layer = 10.0;

    commands
        .spawn((
            ZoomScreenElement,
            Sprite {
                color: overlay_background_color,
                custom_size: Some(full_screen_size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, background_layer),
            Pickable::default(),
        ))
        .observe(close_zoom_screen);
}

fn spawn_input_field(commands: &mut Commands) {
    let input_field_color = Color::srgb(0.9, 0.9, 0.9);
    let input_field_size = Vec2::new(240.0, 60.0);
    let input_field_layer = 11.0;
    let input_field_y = 40.0;
    let input_placeholder_text = "";

    commands.spawn((
        ZoomScreenElement,
        Sprite {
            color: input_field_color,
            custom_size: Some(input_field_size),
            ..default()
        },
        Transform::from_xyz(0.0, input_field_y, input_field_layer),
    ));
    commands.spawn((
        ZoomScreenElement,
        InputFieldDisplay,
        Text2d::new(input_placeholder_text),
        TextColor(Color::BLACK),
        Transform::from_xyz(0.0, input_field_y, input_field_layer + 0.1),
    ));
}

fn spawn_confirm_button(commands: &mut Commands) {
    let confirm_button_color = Color::srgb(0.4, 0.7, 0.4);
    let confirm_button_size = Vec2::new(120.0, 48.0);
    let confirm_button_layer = 11.0;
    let confirm_button_y = -60.0;
    let confirm_button_label = "confirm";

    commands
        .spawn((
            ZoomScreenElement,
            Sprite {
                color: confirm_button_color,
                custom_size: Some(confirm_button_size),
                ..default()
            },
            Transform::from_xyz(0.0, confirm_button_y, confirm_button_layer),
            Pickable::default(),
        ))
        .observe(on_confirm_click);
    commands.spawn((
        ZoomScreenElement,
        Text2d::new(confirm_button_label),
        Transform::from_xyz(0.0, confirm_button_y, confirm_button_layer + 0.1),
    ));
}

fn spawn_feedback_display(commands: &mut Commands) {
    let feedback_layer = 11.0;
    let feedback_y = -120.0;
    let feedback_color = Color::srgb(0.8, 0.2, 0.2);

    commands.spawn((
        ZoomScreenElement,
        FeedbackDisplay,
        Text2d::new(""),
        TextColor(feedback_color),
        Transform::from_xyz(0.0, feedback_y, feedback_layer),
    ));
}

fn on_confirm_click(
    _click: On<Pointer<Click>>,
    input_buffer: Res<InputBuffer>,
    mut commands: Commands,
) {
    let input_text = input_buffer.text.clone();
    let answer = check_quiz1_answer(&input_text);

    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = match answer {
            QuizAnswer::Correct(solution) => vec![Box::new(SetQuiz1Solution { solution })],
            QuizAnswer::Incorrect => vec![Box::new(WrongAnswerMessage { input: input_text })],
        };
        apply_effects(effects, world);
    });
}

pub fn sync_input_field_display(
    input_buffer: Res<InputBuffer>,
    mut texts: Query<&mut Text2d, With<InputFieldDisplay>>,
) {
    if !input_buffer.is_changed() {
        return;
    }

    for mut text in &mut texts {
        text.0 = input_buffer.text.clone();
    }
}

pub fn show_wrong_answer_feedback(
    mut wrong_answers: MessageReader<WrongAnswerMessage>,
    mut texts: Query<&mut Text2d, With<FeedbackDisplay>>,
) {
    for wrong in wrong_answers.read() {
        for mut text in &mut texts {
            text.0 = format!("wrong: {}", wrong.input);
        }
    }
}
