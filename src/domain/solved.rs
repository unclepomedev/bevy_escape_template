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

pub fn resolve_quiz1_effects(input: &str) -> Vec<Box<dyn Effect>> {
    match check_quiz1_answer(input) {
        QuizAnswer::Correct(solution) => vec![Box::new(SetQuiz1Solution { solution })],
        QuizAnswer::Incorrect => vec![Box::new(WrongAnswerMessage {
            input: input.to_string(),
        })],
    }
}

pub fn log_quiz1_changes(solved: Res<Solved>) {
    if solved.is_changed() {
        info!("quiz1 solution is now {:?}", solved.quiz1);
    }
}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy_escape_core::apply_effects;

    fn app_with_quiz1_state() -> App {
        let mut app = App::new();
        app.init_resource::<Solved>();
        app.add_message::<WrongAnswerMessage>();
        app
    }

    #[test]
    fn answering_3_sets_solved_to_answer3() {
        let mut app = app_with_quiz1_state();
        apply_effects(resolve_quiz1_effects("3"), app.world_mut());
        assert_eq!(app.world().resource::<Solved>().quiz1, Solution1::Answer3);
    }

    #[test]
    fn answering_12_sets_solved_to_answer12() {
        let mut app = app_with_quiz1_state();
        apply_effects(resolve_quiz1_effects("12"), app.world_mut());
        assert_eq!(app.world().resource::<Solved>().quiz1, Solution1::Answer12);
    }

    #[test]
    fn wrong_answer_does_not_change_solved_state() {
        let mut app = app_with_quiz1_state();
        apply_effects(resolve_quiz1_effects("2"), app.world_mut());
        assert_eq!(app.world().resource::<Solved>().quiz1, Solution1::Unsolved);
    }

    #[test]
    fn wrong_answer_emits_a_wrong_answer_message_with_the_typed_input() {
        let mut app = app_with_quiz1_state();
        apply_effects(resolve_quiz1_effects("2"), app.world_mut());

        let messages = app.world().resource::<Messages<WrongAnswerMessage>>();
        let mut cursor = messages.get_cursor();
        let received: Vec<_> = cursor.read(messages).collect();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].input, "2");
    }

    #[test]
    fn an_earlier_wrong_answer_does_not_block_a_later_correct_one() {
        let mut app = app_with_quiz1_state();
        apply_effects(resolve_quiz1_effects("2"), app.world_mut());
        apply_effects(resolve_quiz1_effects("3"), app.world_mut());
        assert_eq!(app.world().resource::<Solved>().quiz1, Solution1::Answer3);
    }
}
