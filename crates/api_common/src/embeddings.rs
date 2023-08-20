use lemmy_db_schema::source::post::Post;
use lemmy_utils::utils::markdown::MARKDOWN_PARSER;

pub fn create_embeddings(post: Post) {
  // grab the image urls from markdown body
  let md = MARKDOWN_PARSER.parse(post.body);
  let image_urls = md
    .children
    .iter()
    .filter_map(|token| match token {
        Some(MarkdownToken::Image(image)) => Some(image.src.to_string(),
      _ => None,
    })
    // include the post url, since it might be an image
    .chain(std::iter::once(post.url))
    .collect::<Vec<_>>();
}
