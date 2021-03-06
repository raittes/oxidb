#![deny(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces
)]
#![feature(box_syntax)]

//! # oxidb_storage
//!
//! `oxidb_storage` is the storage abstraction layer for an `oxidb` database system implementation.
//!
//! ## Examples
//!

#[macro_use]
extern crate bitflags;

pub use crate::babylon::BabylonStorage;
use failure::Error;
use oxidb_core::ColumnValueOps;
use std::borrow::Cow;

/// `babylon` is `oxidb`'s default storage engine.
pub mod babylon;

/// `ReadOps` define the basic read-only interface for a storage engine.
pub trait ReadOps {
    /// `ColumnValue` implementation of `ColumnValueOps`
    type ColumnValue: ColumnValueOps;

    /// `iter` Returns an iterator over rows as arrays of `Self::ColumnValue` type items.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Cow<'a, [Self::ColumnValue]>> + 'a>
    where
        [Self::ColumnValue]: std::borrow::ToOwned;
}

/// `WriteOps` define the basic write-only interface for a storage engine.
pub trait WriteOps<'a> {
    /// `ColumnValue` implementation of `ColumnValueOps`
    type ColumnValue: ColumnValueOps;

    /// `insert_row` inserts a new row.
    fn insert<T>(&mut self, row: T) -> Result<(), Error>
    where
        T: ExactSizeIterator,
        T: Iterator<Item = Self::ColumnValue>;
}

/// `StorageFactory` defines the interface to build the storage engine.
pub trait StorageFactory<'a> {
    /// The `StorageOps` implementation.
    type Storage: WriteOps<'a>;

    /// `build` a new storage engine instance.
    fn build() -> Result<Self::Storage, Error>;
}
