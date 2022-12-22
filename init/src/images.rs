use futures::{stream, StreamExt};
use reqwest::Client;

use api::{Error, Image, Quiz, Result, IMAGE_ENDPOINT};

async fn upload_image(image: &mut Image, bearer: String) -> Result<()> {
    let filename = (*std::mem::take(image).url().unwrap()).clone();
    let path = format!("init/images/{filename}");

    let bytes = std::fs::read(&path).unwrap();
    let base64 = base64::encode(&bytes);

    *image = Image::from_base64(base64, Some(path));
    image.upload(bearer).await?;

    tracing::info!("uploaded image {filename}");
    Ok(())
}

// async fn upload_quiz_images(
//     client: &Client,
//     mut quiz: Quiz,
//     bearer: String,
// ) -> Result<Quiz, Error> {
//     upload_image(client, &mut quiz.image, bearer.clone()).await?;
//
//     for round in &mut quiz.rounds {
//         upload_image(client, &mut round.image, bearer.clone()).await?;
//     }
//
//     Ok(quiz)
// }

pub async fn upload_images(quizzes: &mut [Quiz], bearer: String) -> Result<()> {
    let mut images = vec![];
    for quiz in quizzes {
        images.push(&mut quiz.image);
        images.extend(quiz.rounds.iter_mut().map(|r| &mut r.image));
    }

    // let iterator = images.into_iter().map(|image| upload_image(client, image, bearer.clone()));
    // let results = future::join_all(iterator).await;

    let results: Vec<_> = stream::iter(images)
        .map(|img| upload_image(img, bearer.clone()))
        .buffer_unordered(8)
        .collect()
        .await;

    results.into_iter().collect()
}

pub async fn delete_images(token: String) -> Result<()> {
    let url = format!("{IMAGE_ENDPOINT}/reset");

    let _ = Client::new()
        .post(&url)
        .header("Authorization", token)
        .send()
        .await
        .map_err(|_| Error::Unreachable("piximages", url))?
        .error_for_status()
        .map_err(|_| Error::ErrorStatus("piximages"))?;

    Ok(())
}
