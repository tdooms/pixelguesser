use crate::{DraftQuiz, DraftRound, Quiz, Round};

#[derive(Debug, Clone, PartialEq)]
pub struct FullQuiz {
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FullDraftQuiz {
    pub quiz: DraftQuiz,
    pub rounds: Vec<DraftRound>,
}

impl Default for FullDraftQuiz {
    fn default() -> Self {
        FullDraftQuiz { quiz: DraftQuiz::default(), rounds: vec![DraftRound::default()] }
    }
}

impl From<FullQuiz> for FullDraftQuiz {
    fn from(full: FullQuiz) -> Self {
        let rounds = full.rounds.iter().cloned().map(Into::into).collect();
        FullDraftQuiz { rounds, quiz: full.quiz.into() }
    }
}
