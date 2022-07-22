use api::{FullDraftQuiz, Quiz, Round};
use hasura::Object;
use std::fs::File;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<FullDraftQuiz>,
}

async fn convert_image(image: &mut api::Image) {
    let path = std::mem::take(image).url().unwrap();

    let vec = std::fs::read(&format!("init/images/{path}")).unwrap();
    *image = api::Image::from_base64(base64::encode(vec), None);

    log::info!("uploading image: {path}");
    image.upload().await.unwrap();
}

async fn upload(admin: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let Quizzes { mut quizzes } = serde_json::from_reader(File::open("init/create.json")?)?;
    let creator = "auth0|61eb4f90e3123a0069a0d0a0";

    for full in &mut quizzes {
        full.quiz.creator_id = creator.to_owned();
        convert_image(&mut full.quiz.image).await;

        for round in &mut full.rounds {
            convert_image(&mut round.image).await;
        }
    }

    let drafts: Vec<_> = quizzes.iter_mut().map(|full| std::mem::take(&mut full.quiz)).collect();

    let quiz_insert =
        hasura::InsertBuilder::default().returning(Quiz::all()).objects(drafts).build().unwrap();
    let inserted = hasura::mutation!(quiz_insert)
        .admin(admin.clone())
        .send(api::GRAPHQL_ENDPOINT)
        .await
        .unwrap();
    let quiz_ids: Vec<_> = inserted.into_iter().map(|x| x.id).collect();

    let rounds: Vec<_> = quizzes
        .into_iter()
        .zip(quiz_ids.iter())
        .flat_map(|(full, quiz_id)| {
            full.rounds
                .into_iter()
                .enumerate()
                .map(|(idx, draft)| Round::from_draft(draft, *quiz_id, idx as u32))
        })
        .collect();
    let round_insert =
        hasura::InsertBuilder::default().returning(Round::all()).objects(rounds).build().unwrap();

    let _ = hasura::mutation!(round_insert)
        .admin(admin.clone())
        .send(api::GRAPHQL_ENDPOINT)
        .await
        .unwrap();

    log::info!("uploaded the following quizzes: {:?}", quiz_ids);

    Ok(())
}

async fn delete(admin: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let delete = hasura::DeleteBuilder::default().returning(Quiz::all()).build().unwrap();
    let quizzes = hasura::mutation!(delete).admin(admin).send(api::GRAPHQL_ENDPOINT).await.unwrap();

    let info: Vec<_> = quizzes.into_iter().map(|x| x.title).collect();
    log::warn!("deleted the following quizzes: {info:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let admin = std::env::var("ADMIN_TOKEN").ok();

    delete(admin.clone()).await?;
    upload(admin).await
}
