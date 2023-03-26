pub mod get;
pub mod post;

pub use self::get::openai_get;
pub use self::post::{openai_post, openai_post_form};
