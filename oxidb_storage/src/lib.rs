#[macro_use]
extern crate bitflags;
extern crate failure;
extern crate log;
extern crate oxidb_core;

pub mod babylon;

use failure::Error;
use std::borrow::Cow;

pub trait PageInfo {
    fn get_free_space(&self) -> usize;
    fn get_page_size(&self) -> usize;
    fn get_row_count(&self) -> usize;
}

pub trait PageOps {
    type ColumnValue: Clone;

    fn iter<'b>(&'b self) -> Box<dyn Iterator<Item = Cow<'b, [Self::ColumnValue]>> + 'b>;
    fn insert<T>(&mut self, row: T) -> Result<(), Error>
    where
        T: ExactSizeIterator,
        T: Iterator<Item = Self::ColumnValue>;
}