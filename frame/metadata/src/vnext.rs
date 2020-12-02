// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Decodable variant of the RuntimeMetadata.
//!
//! This really doesn't belong here, but is necessary for the moment. In the future
//! it should be removed entirely to an external module for shimming on to the
//! codec-encoded metadata.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use codec::{Encode, Decode};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

use scale_info::{
	form::{
		CompactForm,
		Form,
		MetaForm,
	},
	meta_type,
	IntoCompact,
	Registry,
	TypeInfo,
};

pub type RuntimeMetadataLastVersion<T> = RuntimeMetadataV11<T>;

/// Metadata prefixed by a u32 for reserved usage
#[derive(Eq, Encode, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct RuntimeMetadataPrefixed<T: Form = MetaForm>(pub u32, pub RuntimeMetadata<T>);

impl From<RuntimeMetadataLastVersion<CompactForm>> for RuntimeMetadataPrefixed<CompactForm> {
	fn from(metadata: RuntimeMetadataLastVersion<CompactForm>) -> RuntimeMetadataPrefixed<CompactForm> {
		RuntimeMetadataPrefixed(super::META_RESERVED, RuntimeMetadata::V11(metadata))
	}
}

impl From<RuntimeMetadataPrefixed<CompactForm>> for sp_core::OpaqueMetadata {
	fn from(metadata: RuntimeMetadataPrefixed<CompactForm>) -> Self {
		sp_core::OpaqueMetadata::new(metadata.encode())
	}
}

/// The metadata of a runtime.
/// The version ID encoded/decoded through
/// the enum nature of `RuntimeMetadata`.
#[derive(Eq, Encode, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub enum RuntimeMetadata<T: Form = MetaForm> {
	/// Version 11 for runtime metadata.
	V11(RuntimeMetadataV11<T>),
}

/// The metadata of a runtime.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct RuntimeMetadataV11<T: Form = MetaForm> {
	/// Metadata of all the modules.
	pub modules: Vec<ModuleMetadata<T>>,
	// /// Metadata of the extrinsic.
	// pub extrinsic: ExtrinsicMetadata<F>,
}

impl IntoCompact for RuntimeMetadataV11 {
	type Output = RuntimeMetadataV11<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		RuntimeMetadataV11 {
			modules: registry.map_into_compact(self.modules),
			// extrinsic: self.extrinsic.into_compact(registry),
		}
	}
}

/// Metadata of the extrinsic used by the runtime.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct ExtrinsicMetadata<T: Form = MetaForm> {
	/// Extrinsic version.
	pub version: u8,
	/// The signed extensions in the order they appear in the extrinsic.
	pub signed_extensions: Vec<T::Type>,
}

impl IntoCompact for ExtrinsicMetadata {
	type Output = ExtrinsicMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		ExtrinsicMetadata {
			version: self.version,
			signed_extensions: registry.register_types(self.signed_extensions),
		}
	}
}

/// All metadata about an runtime module.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct ModuleMetadata<T: Form = MetaForm> {
	pub name: T::String,
	// pub storage: Option<DecodeDifferent<FnEncode<StorageMetadata>, StorageMetadata>>,
	pub calls: Option<Vec<FunctionMetadata<T>>>,
	pub event: Option<Vec<EventMetadata<T>>>,
	// pub constants: DFnA<ModuleConstantMetadata>,
	// pub errors: DFnA<ErrorMetadata>,
}

impl IntoCompact for ModuleMetadata {
	type Output = ModuleMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		ModuleMetadata {
			name: self.name.into_compact(registry),
			calls: self.calls.map(|calls| registry.map_into_compact(calls)),
			event: self.event.map(|event| registry.map_into_compact(event)),
		}
	}
}

/// All the metadata about a function.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct FunctionMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub arguments: Vec<FunctionArgumentMetadata<T>>,
	pub documentation: Vec<T::String>,
}

impl IntoCompact for FunctionMetadata {
	type Output = FunctionMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		FunctionMetadata {
			name: self.name.into_compact(registry),
			arguments: registry.map_into_compact(self.arguments),
			documentation: registry.map_into_compact(self.documentation),
		}
	}
}

/// All the metadata about a function argument.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct FunctionArgumentMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
	pub is_compact: bool,
}

impl IntoCompact for FunctionArgumentMetadata {
	type Output = FunctionArgumentMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		FunctionArgumentMetadata {
			name: self.name.into_compact(registry),
			ty: registry.register_type(&self.ty),
			is_compact: self.is_compact,
		}
	}
}

/// All the metadata about an outer event.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct OuterEventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub events: Vec<ModuleEventMetadata<T>>,
}

impl IntoCompact for OuterEventMetadata {
	type Output = OuterEventMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		OuterEventMetadata {
			name: self.name.into_compact(registry),
			events: registry.map_into_compact(self.events),
		}
	}
}

/// Metadata about a module event.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct ModuleEventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub events: Vec<EventMetadata<T>>,
}

impl IntoCompact for ModuleEventMetadata {
	type Output = ModuleEventMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		ModuleEventMetadata {
			name: self.name.into_compact(registry),
			events: registry.map_into_compact(self.events),
		}
	}
}

/// All the metadata about an event.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct EventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub arguments: Vec<TypeSpec<T>>,
	pub documentation: Vec<T::String>,
}

impl IntoCompact for EventMetadata {
	type Output = EventMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		EventMetadata {
			name: self.name.into_compact(registry),
			arguments: registry.map_into_compact(self.arguments),
			documentation: registry.map_into_compact(self.documentation),
		}
	}
}

/// Describes the syntactical name of a type at a given type position.
///
/// This is important when trying to work with type aliases.
/// Normally a type alias is transparent and so scenarios such as
/// ```no_compile
/// type Foo = i32;
/// fn bar(foo: Foo);
/// ```
/// Will only communicate that `foo` is of type `i32` which is correct,
/// however, it will miss the potentially important information that it
/// is being used through a type alias named `Foo`.
pub type DisplayName<T> = scale_info::Path<T>;

/// A type specification.
///
/// This contains the actual type as well as an optional compile-time
/// known displayed representation of the type. This is useful for cases
/// where the type is used through a type alias in order to provide
/// information about the alias name.
///
/// # Examples
///
/// Consider the following Rust function:
/// ```no_compile
/// fn is_sorted(input: &[i32], pred: Predicate) -> bool;
/// ```
/// In this above example `input` would have no displayable name,
/// `pred`'s display name is `Predicate` and the display name of
/// the return type is simply `bool`. Note that `Predicate` could
/// simply be a type alias to `fn(i32, i32) -> Ordering`.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub struct TypeSpec<T: Form = MetaForm> {
	/// The actual type.
	ty: T::Type,
	/// The compile-time known displayed representation of the type.
	display_name: DisplayName<T>,
}

impl IntoCompact for TypeSpec {
	type Output = TypeSpec<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		TypeSpec {
			ty: registry.register_type(&self.ty),
			display_name: self.display_name.into_compact(registry),
		}
	}
}

impl TypeSpec {
	/// Creates a new type specification with a display name.
	///
	/// The name is any valid Rust identifier or path.
	///
	/// # Examples
	///
	/// Valid display names are `foo`, `foo::bar`, `foo::bar::Baz`, etc.
	///
	/// # Panics
	///
	/// Panics if the given display name is invalid.
	pub fn with_name_str<T>(display_name: &'static str) -> Self
		where
			T: TypeInfo + 'static,
	{
		Self::with_name_segs::<T, _>(display_name.split("::"))
	}

	/// Creates a new type specification with a display name
	/// represented by the given path segments.
	///
	/// The display name segments all must be valid Rust identifiers.
	///
	/// # Examples
	///
	/// Valid display names are `foo`, `foo::bar`, `foo::bar::Baz`, etc.
	///
	/// # Panics
	///
	/// Panics if the given display name is invalid.
	pub fn with_name_segs<T, S>(segments: S) -> Self
		where
			T: TypeInfo + 'static,
			S: IntoIterator<Item = &'static str>,
	{
		Self {
			ty: meta_type::<T>(),
			display_name: DisplayName::from_segments(segments)
				.expect("display name is invalid"),
		}
	}

	/// Creates a new type specification without a display name.
	pub fn new<T>() -> Self
		where
			T: TypeInfo + 'static,
	{
		Self {
			ty: meta_type::<T>(),
			display_name: DisplayName::default(),
		}
	}
}
