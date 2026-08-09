use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const DEFAULT_BATTLE_QUESTION_COUNT: usize = 7;

#[derive(Clone, Deserialize, Serialize)]
pub struct Question {
    pub word: String,
    pub options: [String; 4],
    #[serde(skip_serializing)]
    pub answer: String,
}

static QUESTIONS: OnceLock<Vec<Question>> = OnceLock::new();

pub fn init() {
    questions();
}

pub fn question_at(index: usize) -> Question {
    let questions = questions();
    questions[index % questions.len()].clone()
}

pub fn question_count() -> usize {
    questions().len()
}

pub fn battle_question_count() -> usize {
    DEFAULT_BATTLE_QUESTION_COUNT
}

fn questions() -> &'static [Question] {
    QUESTIONS.get_or_init(|| {
        let questions: Vec<Question> =
            serde_json::from_str(include_str!("../data/questions.json")).expect("valid questions.json");

        assert!(!questions.is_empty(), "questions.json must contain questions");
        for question in &questions {
            assert!(
                question.options.contains(&question.answer),
                "answer must be one of the options for {}",
                question.word
            );
        }

        questions
    })
}
