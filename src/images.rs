use std::io::Cursor;

use image::ImageReader;

pub async fn print(img_url: &str) {
  let bytes = match reqwest::get(img_url)
    .await
    .and_then(|r| r.error_for_status())
  {
    Ok(resp) => match resp.bytes().await {
      Ok(b) => b,
      Err(e) => {
        eprintln!("Failed to read image bytes: {}", e);
        return;
      }
    },
    Err(e) => {
      eprintln!("Failed to fetch image: {}", e);
      return;
    }
  };

  let img = match ImageReader::new(Cursor::new(bytes)).with_guessed_format() {
    Ok(reader) => match reader.decode() {
      Ok(img) => img,
      Err(e) => {
        eprintln!("Failed to decode image: {}", e);
        return;
      }
    },
    Err(e) => {
      eprintln!("Failed to read image format: {}", e);
      return;
    }
  };

  if let Err(e) = viuer::print(
    &img,
    &viuer::Config {
      width: Some(60),
      ..Default::default()
    },
  ) {
    eprintln!("Failed to display image: {}", e);
  }
}
