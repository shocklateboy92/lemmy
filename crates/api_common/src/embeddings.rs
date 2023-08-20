use image_hasher::{Hasher, HasherConfig, ImageHash};
use lemmy_db_schema::{
  schema::embedding,
  source::{embedding::Embedding, post::Post},
};
use lemmy_utils::{error::LemmyError, utils::markdown::MARKDOWN_PARSER};
use markdown_it::plugins::cmark::inline::image::Image;
use reqwest::header::CONTENT_TYPE;
use reqwest_middleware::ClientWithMiddleware;
use tracing::info;
use url::Url;

use crate::context::LemmyContext;

pub async fn create_embeddings(post: Post, context: &LemmyContext) {
  let hasher: Hasher = HasherConfig::new().to_hasher();
  let mut image_urls = Vec::<Url>::new();

  // grab the images from the body if there are any
  if let Some(body) = post.body {
    let md = MARKDOWN_PARSER.parse(&body);
    md.walk(|node, depth| {
      if let Some(image) = node.cast::<Image>() {
        if let Ok(url) = Url::parse(&image.url) {
          image_urls.push(url);
        }
      }
    });
  }

  // the post itself might be an image
  if let Some(url) = post.url {
    image_urls.push(url.into());
  }

  // download the images
  let images = futures::future::join_all(image_urls.into_iter().map(|image| async {
    (
      image.clone(),
      fetch_and_hash_image(context.client(), &hasher, image).await,
    )
  }))
  .await;

  let hashes = images.into_iter().filter_map(|image| match image {
    (_, Ok(hash)) => Some(hash),
    (image, Err(e)) => {
      info!("ignoring repost checking for image '{}': {}", image, e);
      None
    }
  });

  // diesel::insert_into(embedding::table)
  //   .values(Embedding::new(post.id, hash)
  //   .execute(context.pool())
  hashes.into_iter().map(|hash| Embedding {
    id: post.id,
    embedding: pgvector::Vector::from(hash.to_bytes()),
  });
}

async fn fetch_and_hash_image(
  client: &ClientWithMiddleware,
  hasher: &Hasher,
  url: Url,
) -> Result<ImageHash, LemmyError> {
  let response = client.execute(client.get(url).build()?).await?;

  // make sure the content-type is an image
  if !response
    .headers()
    .get(CONTENT_TYPE)
    .ok_or(LemmyError::from_message(
      "non-ASCII characters in content-type header",
    ))?
    .to_str()?
    .starts_with("image/")
  {
    return Err(LemmyError::from_message("non-image content-type header"));
  }

  let bytes = response.bytes().await?;
  let image = image::load_from_memory(&bytes)?;
  let hash = hasher.hash_image(&image);

  Ok(hash)
}
