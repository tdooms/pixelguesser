#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "quizzes", pk = "id", draft = "DraftQuiz")]
pub struct Quiz {
    pub id: u32,
    pub public: bool,
    pub complete: bool,
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    pub image: Image,

    #[object(expand)]
    pub creator: User,

    #[object(expand)]
    #[serde(default)]
    pub tags: Vec<Tag>,

    #[object(expand)]
    #[serde(default)]
    pub rounds: Vec<Round>,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DraftQuiz {
    #[validate(length(min = 1, message = "Quiz must have a title."))]
    #[validate(length(max = 32, message = "Title cannot exceed 32 characters."))]
    pub title: String,

    #[serde(default)]
    pub creator_id: Option<String>,

    pub description: String,
    pub explanation: String,
    pub public: bool,

    #[serde(default)]
    pub image: Image,

    #[serde(skip_serializing_if = "skip_empty")]
    #[serde(deserialize_with = "strip_data")]
    #[serde(default)]
    pub tags: Data<Vec<DraftTag>>,

    #[serde(skip_serializing_if = "skip_empty")]
    #[serde(deserialize_with = "strip_data")]
    #[serde(default)]
    pub rounds: Data<Vec<DraftRound>>,
}
