use cobul::Form;
use crate::graphql::DraftQuiz;

pub enum Msg {
    Changed(DraftQuiz),
    Submit(DraftQuiz)
    Cancel
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Form<DraftQuiz, Msg>) {

}