//! Glue code and convenience methods between [`rhizome`] and [`fruit_salad`].
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fmayonnaise)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fmayonnaise)

#![doc(html_root_url = "https://docs.rs/mayonnaise/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned, clippy::type_complexity)]
#![no_std]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use core::{any::TypeId, borrow::Borrow, pin::Pin};

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
pub impl<T, K: Ord, C: RefCounter> Node<T, K, dyn Dyncast, C>
where
	Self: Sealed,
{
	/// Gets a value from this [`Node`] or one of its ancestors and attempts to cast it to a certain type.
	fn get_dynamic<Q: ?Sized, V: ?Sized>(
		&self,
		key: &Q,
	) -> Option<(&Self, Result<Pin<&V>, Pin<&dyn Dyncast>>)>
	where
		K: Borrow<Q>,
		Q: Ord,
		V: 'static,
	{
		let (node, value) = self.get(key)?;
		Some((node, value.dyncast_pinned::<V>().ok_or(value)))
	}

	/// Gets a value from this [`Node`] or one of this ancestors and attempts to cast it to a certain type.
	///
	/// # Safety
	///
	/// `VActual` and `VStatic` must be the same type except for lifetimes.
	///
	/// `VActual` must not be longer-lived than `Self`.
	unsafe fn get_dynamic_<Q: ?Sized, VActual: ?Sized, VStatic: ?Sized>(
		&self,
		key: &Q,
	) -> Option<(&Self, Result<Pin<&VActual>, Pin<&dyn Dyncast>>)>
	where
		K: Borrow<Q>,
		Q: Ord,
		VStatic: 'static,
	{
		let (node, value) = self.get(key)?;
		Some((
			node,
			value.dyncast_pinned_::<VActual, VStatic>().ok_or(value),
		))
	}

	/// Gets a value from this [`Node`] and attempts to cast it to a certain type.
	fn get_local_dynamic<Q: ?Sized, V: ?Sized>(
		&self,
		key: &Q,
	) -> Option<Result<Pin<&V>, Pin<&dyn Dyncast>>>
	where
		K: Borrow<Q>,
		Q: Ord,
		V: 'static,
	{
		self.get_local(key)
			.map(|value| value.dyncast_pinned::<V>().ok_or(value))
	}

	/// Gets a value from this [`Node`] and attempts to cast it to a certain type.
	///
	/// # Safety
	///
	/// `VActual` and `VStatic` must be the same type except for lifetimes.
	///
	/// `VActual` must not be longer-lived than `Self`.
	unsafe fn get_local_dynamic_<Q: ?Sized, VActual: ?Sized, VStatic: ?Sized>(
		&self,
		key: &Q,
	) -> Option<Result<Pin<&VActual>, Pin<&dyn Dyncast>>>
	where
		K: Borrow<Q>,
		Q: Ord,
		VStatic: 'static,
	{
		self.get_local(key)
			.map(|value| value.dyncast_pinned_::<VActual, VStatic>().ok_or(value))
	}
}

/// Extension methods for working with a [`Node`] that acts as instance registry by type.
#[ext(InstanceRegistryNodeExt)]
pub impl<T, C: RefCounter> Node<T, TypeId, dyn Dyncast, C>
where
	Self: Sealed,
{
	/// Gets a value from this [`Node`] or one of its ancestors by `V`'s [`TypeId`] and attempts to cast it to `V`.
	fn get_instance<V: ?Sized>(&self) -> Option<(&Self, Result<Pin<&V>, Pin<&dyn Dyncast>>)>
	where
		V: 'static,
	{
		let (node, value) = self.get(&TypeId::of::<V>())?;
		Some((node, value.dyncast_pinned::<V>().ok_or(value)))
	}

	/// Gets a value from this [`Node`] or one of this ancestors  by `VStatic`'s [`TypeId`] and attempts to cast it to `VActual`.
	///
	/// # Safety
	///
	/// `VActual` and `VStatic` must be the same type except for lifetimes.
	///
	/// `VActual` must not be longer-lived than `Self`.
	unsafe fn get_instance_<VActual: ?Sized, VStatic: ?Sized>(
		&self,
	) -> Option<(&Self, Result<Pin<&VActual>, Pin<&dyn Dyncast>>)>
	where
		VStatic: 'static,
	{
		let (node, value) = self.get(&TypeId::of::<VStatic>())?;
		Some((
			node,
			value.dyncast_pinned_::<VActual, VStatic>().ok_or(value),
		))
	}

	/// Gets a value from this [`Node`] by `V`'s [`TypeId`] and attempts to cast it to `V`.
	fn get_local_instance<V: ?Sized>(&self) -> Option<Result<Pin<&V>, Pin<&dyn Dyncast>>>
	where
		V: 'static,
	{
		self.get_local(&TypeId::of::<V>())
			.map(|value| value.dyncast_pinned::<V>().ok_or(value))
	}

	/// Gets a value from this [`Node`] by `VStatic`'s [`TypeId`] and attempts to cast it to `VActual`.
	///
	/// # Safety
	///
	/// `VActual` and `VStatic` must be the same type except for lifetimes.
	///
	/// `VActual` must not be longer-lived than `Self`.
	unsafe fn get_local_instance_<Q: ?Sized, VActual: ?Sized, VStatic: ?Sized>(
		&self,
	) -> Option<Result<Pin<&VActual>, Pin<&dyn Dyncast>>>
	where
		VStatic: 'static,
	{
		self.get_local(&TypeId::of::<VStatic>())
			.map(|value| value.dyncast_pinned_::<VActual, VStatic>().ok_or(value))
	}
}
