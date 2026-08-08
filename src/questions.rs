use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Question {
    pub word: &'static str,
    pub options: [&'static str; 4],
    #[serde(skip_serializing)]
    pub answer: &'static str,
}

const QUESTIONS: [Question; 5] = [
    Question {
        word: "Ephemeral",
        options: [
            "Lasts forever",
            "Exists briefly",
            "Very dangerous",
            "Easy to understand",
        ],
        answer: "Exists briefly",
    },
    Question {
        word: "Benevolent",
        options: ["Kind", "Cruel", "Fast", "Weak"],
        answer: "Kind",
    },
    Question {
        word: "Meticulous",
        options: ["Careful", "Lazy", "Aggressive", "Funny"],
        answer: "Careful",
    },
    Question {
        word: "Obsolete",
        options: ["Modern", "Outdated", "Beautiful", "Expensive"],
        answer: "Outdated",
    },
    Question {
        word: "Ambiguous",
        options: ["Clear", "Uncertain", "Powerful", "Helpful"],
        answer: "Uncertain",
    },
];

pub fn question_at(index: usize) -> Question {
    QUESTIONS[index % QUESTIONS.len()].clone()
}

pub fn question_count() -> usize {
    QUESTIONS.len()
}
