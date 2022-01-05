use crate::error::Error;
use crate::structs::Image;

pub fn upload_image(image: &Image) -> Result<Image, Error> {
    // TODO: upload the image
    Ok(Image::from_url("random url"))
}
