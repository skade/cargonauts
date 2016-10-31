use std::result;
use std::str::FromStr;

use repr::Represent;

mod async;
mod error;
mod get;
mod delete;
mod index;
mod patch;
mod post;
mod sort;

pub mod raw;
pub mod rel;

pub use self::async::AsyncJob;
pub use self::error::Error;
pub use self::get::Get;
pub use self::delete::Delete;
pub use self::index::Index;
pub use self::patch::{Patch, PatchAsync};
pub use self::post::{Post, PostAsync};

pub trait Resource: Represent + Sized + 'static {
    type Id: ToString + FromStr + PartialEq + Clone;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
    fn resource_plural() -> &'static str;
}

pub enum Entity<T: Resource> {
    Id(T::Id),
    Resource(T),
}

pub type Result<T> = result::Result<T, Error>;