use serde_with::skip_serializing_none;

use crate::schema::{embedding, post_embedding};

#[skip_serializing_none]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = embedding))]
pub struct Embedding {
  pub id: i32,
  pub embedding: pgvector::Vector,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "full", derive(Queryable))]
#[cfg_attr(feature = "full", diesel(table_name = post_embedding))]
pub struct PostEmbedding {
  pub post_id: i32,
  pub embedding_id: i32,
}
