use api::{DraftQuiz, Quiz, GRAPHQL_ENDPOINT};
use hasura::{mutation, DeleteBuilder, InsertBuilder, Object};
use std::fs::File;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<DraftQuiz>,
}

async fn convert_image(image: &mut api::Image, creator: String) {
    let path = std::mem::take(image).url().unwrap();

    let vec = std::fs::read(&format!("init/images/{path}")).unwrap();
    *image = api::Image::from_base64(base64::encode(vec), None);

    log::info!("uploading image: {path}");
    image.upload(creator).await.unwrap();
}

async fn upload(admin: Option<String>, creator: String) -> Result<(), Box<dyn std::error::Error>> {
    let Quizzes { mut quizzes } = serde_json::from_reader(File::open("init/create.json")?)?;

    for draft in &mut quizzes {
        draft.creator_id = creator.to_owned();
        convert_image(&mut draft.image, creator.clone()).await;

        for (index, round) in &mut draft.rounds.data.iter_mut().enumerate() {
            convert_image(&mut round.image, creator.clone()).await;
            round.index = index as u32
        }
    }

    let insert = InsertBuilder::default().returning(Quiz::all()).objects(quizzes).build().unwrap();
    let inserted = mutation!(insert).admin(admin.clone()).send(GRAPHQL_ENDPOINT).await.unwrap();

    let quiz_ids: Vec<_> = inserted.into_iter().map(|x| x.id).collect();
    log::info!("uploaded the following quizzes: {:?}", quiz_ids);

    Ok(())
}

async fn delete(admin: Option<String>, creator: String) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: also remove images from storage

    let delete = DeleteBuilder::default().returning(Quiz::all()).build().unwrap();
    let quizzes = mutation!(delete).admin(admin).send(GRAPHQL_ENDPOINT).await.unwrap();

    let info: Vec<_> = quizzes.into_iter().map(|x| x.title).collect();
    log::warn!("deleted the following quizzes: {info:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let admin = Some(std::env::var("ADMIN_TOKEN").unwrap());
    let creator = std::env::var("CREATOR").unwrap();

    delete(admin.clone(), creator.clone()).await?;
    upload(admin, creator).await
}
