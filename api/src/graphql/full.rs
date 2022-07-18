use crate::{DraftQuiz, DraftRound, Quiz, Round};

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FullQuiz {
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct FullDraftQuiz {
    #[serde(flatten)]
    pub quiz: DraftQuiz,
    pub rounds: Vec<DraftRound>,
}

impl From<FullQuiz> for FullDraftQuiz {
    fn from(full: FullQuiz) -> Self {
        let rounds = full.rounds.iter().cloned().map(Into::into).collect();
        FullDraftQuiz { rounds, quiz: full.quiz.into() }
    }
}
