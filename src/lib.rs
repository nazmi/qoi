pub mod header;
pub mod encode;
pub mod pixel;
pub mod error;
pub mod types;
pub mod consts;


pub use crate::encode::Encoder;
pub use crate::error::{Error, Result};
pub use crate::header::Header;
pub use crate::types::{Channels, ColorSpace};
