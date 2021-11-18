//! Glue code and convenience methods between [`rhizome`] and [`fruit_salad`].
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fmayonnaise)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fmayonnaise)

#![doc(html_root_url = "https://docs.rs/mayonnaise/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![no_std]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use easy_ext::ext;
use fruit_salad::Dyncast;
use private::Sealed;
use rhizome::sync::Node;
use tiptoe::RefCounter;

/// Prelude, import with `use mayonnaise::prelude::*;`.
pub mod prelude {
	pub use crate::{InstanceRegistryNodeExt, MixedNodeExt};
}

mod private {
	use rhizome::sync::Node;
	use tiptoe::RefCounter;

	pub trait Sealed {}
	impl<T, K: Ord, V, C: RefCounter> Sealed for Node<T, K, V, C> {}
}

/// Extension methods for working with [`Node`]s with dynamically-typed content.
#[ext(MixedNodeExt)]
pub impl<T, K: Ord, C: RefCounter> Node<T, K, dyn Dyncast, C> where Self: Sealed {}

/// Extension methods for working with a [`Node`] that acts as instance registry by type.
#[ext(InstanceRegistryNodeExt)]
pub impl<T, K: Ord, V, C: RefCounter> Node<T, K, V, C> where Self: Sealed {}
