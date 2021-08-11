#[derive(Deserialize, Debug)]
pub struct Query<T> {
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginate<T> {
    data: Vec<T>,
    before: Option<String>,
    after: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quizzes {
    quizzes: Paginate<Quiz>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindQuiz {
    #[serde(rename(deserialize = "findQuizByID"))]
    quiz: QuizWithRounds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizWithRounds {
    #[serde(flatten)]
    quiz: Quiz,

    rounds: Paginate<Round>,
}