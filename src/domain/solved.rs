use bevy::prelude::*;
use bevy_escape_core::Effect;

#[derive(Resource, Default)]
pub struct Solved {
    pub quiz1: Solution1,
}

#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub enum Solution1 {
    #[default]
    Unsolved,
    Answer3,
    Answer12,
}

pub enum QuizAnswer {
    Correct(Solution1),
    Incorrect,
}

pub fn check_quiz1_answer(input: &str) -> QuizAnswer {
    match input {
        "3" => QuizAnswer::Correct(Solution1::Answer3),
        "12" => QuizAnswer::Correct(Solution1::Answer12),
        _ => QuizAnswer::Incorrect,
    }
}

pub struct SetQuiz1Solution {
    pub solution: Solution1,
}

impl Effect for SetQuiz1Solution {
    fn apply(self: Box<Self>, world: &mut World) {
        world.resource_mut::<Solved>().quiz1 = self.solution;
    }
}

#[derive(Message, Clone)]
pub struct WrongAnswerMessage {
    pub input: String,
}

pub fn log_quiz1_changes(solved: Res<Solved>) {
    if solved.is_changed() {
        info!("quiz1 solution is now {:?}", solved.quiz1);
    }
}
