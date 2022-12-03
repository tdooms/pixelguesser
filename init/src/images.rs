use std::error::Error;

use reqwest::{Client, Response};

use api::{Image, Quiz, UPLOAD_ENDPOINT};

async fn convert_image(image: &mut Image, bearer: String) {
    let filename = (*std::mem::take(image).url().unwrap()).clone();
    let path = format!("init/images/{filename}");

    log::info!("uploading: {path}");

    let bytes = std::fs::read(&path).unwrap();
    let base64 = base64::encode(&bytes);

    *image = Image::from_base64(base64, Some(path));
    image.upload(bearer).await.unwrap();
}

pub async fn upload_images(quizzes: &mut [Quiz], bearer: String) {
    // TODO: optimize by only using one client, does this work?
    // let client = Client::new();

    // upload all quiz images
    for quiz in quizzes.iter_mut() {
        convert_image(&mut quiz.image, bearer.clone()).await;
    }

    // upload all round images
    for round in quizzes.iter_mut().map(|q| q.rounds.iter_mut()).flatten() {
        convert_image(&mut round.image, bearer.clone()).await;
    }
}

pub async fn delete_images(token: String) -> Result<Response, Box<dyn Error>> {
    let endpoint = format!("{UPLOAD_ENDPOINT}/reset");

    let response = Client::new().post(&endpoint).header("Authorization", token).send().await?;
    Ok(response)
}
