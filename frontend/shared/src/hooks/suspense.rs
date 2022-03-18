use api::{full_quiz, quizzes, FullQuiz, Quiz, User};
use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::suspense::{Suspension, SuspensionResult};
use yew::{hook, use_state, UseStateHandle};

#[hook]
pub fn use_async_suspension<T: Clone + 'static, F: Future<Output = T> + 'static>(
    fut: F,
) -> SuspensionResult<T> {
    let state: UseStateHandle<Option<T>> = use_state(|| None);
    let cloned = state.clone();

    match &*state {
        Some(data) => Ok(data.clone()),
        None => {
            let (s, handle) = Suspension::new();
            spawn_local(async move {
                cloned.set(Some(fut.await));
                handle.resume();
            });
            Err(s)
        }
    }
}

#[hook]
pub fn use_quizzes(user: Option<User>) -> SuspensionResult<Vec<Quiz>> {
    use_async_suspension(async move { quizzes(user).await.unwrap() })
}

#[hook]
pub fn use_full_quiz(user: Option<User>, id: u64) -> SuspensionResult<FullQuiz> {
    use_async_suspension(async move { full_quiz(user, id).await.unwrap() })
}
